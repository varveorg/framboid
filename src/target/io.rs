use std::{fs::{File, OpenOptions, read_dir, rename}, io::{Error, ErrorKind, Read, Result, Write}, net::{SocketAddr, TcpStream, UdpSocket}};

use blake3::hash;
use interprocess::local_socket::{ConnectOptions, Name, Stream};
use reqwest::blocking::Client;

use crate::target::{Target, TargetKind, endpoint::EndpointKind};

impl Target {
    /// A reading abstraction over `TargetKind` that fills a buffer.
    pub fn read(&self, buffer: &mut Vec<u8>) -> Result<()> {
        match &self.0 {
            TargetKind::Directory(dir) => {
                for entry in read_dir(dir)? {
                    let entry = entry?;
                    let path = entry.path();
                    
                    if !entry.file_type()?.is_file() {
                        return Err(Error::new(ErrorKind::InvalidData, "Nested directories are not allowed."))
                    }

                    if let Some(ext) = path.extension() && ext == "tmp" {
                        continue;
                    }

                    let mut file = File::open(path)?;
                    file.read_to_end(buffer)?;
                }
            }
            TargetKind::Endpoint(ep) => {
                match &ep.0 {
                    EndpointKind::Http(url) => {
                        let mut response = Client::new()
                            .get(url.as_str())
                            .send()
                            .map_err(Error::other)?
                            .error_for_status()
                            .map_err(Error::other)?;

                        response.read_to_end(buffer)?;
                    }
                    EndpointKind::Tcp(addr) => {
                        let mut stream = TcpStream::connect(addr)?;

                        stream.read_to_end(buffer)?;
                    }
                    EndpointKind::Udp(addr) => {
                        let addr = addr.parse::<SocketAddr>().map_err(|err| Error::new(ErrorKind::InvalidInput, err))?;

                        let socket = UdpSocket::bind(addr)?;

                        let mut datagram = vec![0; 65536];
                        let received = socket.recv(&mut datagram)?;

                        buffer.extend(&datagram[..received]);
                    }
                }
            }
            TargetKind::File(file) => {
                let mut file = File::open(file)?;
                file.lock_shared()?;

                file.read_to_end(buffer)?;
            }
            TargetKind::Socket(name) => {
                let mut stream = connect(name)?;

                stream.read_to_end(buffer)?;
            }
        }

        Ok(())
    }

    /// A writing abstraction over `TargetKind` that consumes a buffer.
    pub fn write(&self, buffer: Vec<u8>) -> Result<()> {
        match &self.0 {
            TargetKind::Directory(dir) => {
                let name = hash(&buffer).to_string();
                
                let tmp = dir.join(format!("{}.tmp", name));
                let mut file = File::create_new(&tmp)?;
                
                file.write_all(&buffer)?;
                
                rename(tmp, dir.join(name))?;
            }
            TargetKind::Endpoint(ep) => {
                match &ep.0 {
                    EndpointKind::Http(url) => {
                        Client::new()
                            .post(url.as_str())
                            .body(buffer)
                            .send()
                            .map_err(Error::other)?;
                    }
                    EndpointKind::Tcp(addr) => {
                        let mut stream = TcpStream::connect(addr)?;
                        
                        stream.write_all(&buffer)?;
                    }
                    EndpointKind::Udp(addr) => {
                        let addr = addr.parse::<SocketAddr>().map_err(|err| Error::new(ErrorKind::InvalidInput, err))?;
                        
                        let bind = match addr {
                            SocketAddr::V4(_) => "0.0.0.0:0",
                            SocketAddr::V6(_) => "[::]:0"
                        };

                        let socket = UdpSocket::bind(bind)?;
                        socket.connect(addr)?;

                        let sent = socket.send(&buffer)?;
                        if sent != buffer.len() {
                            return Err(Error::new(ErrorKind::WriteZero, format!("Incomplete UDP send. {} bytes were sent.", sent)));
                        }
                    }
                }
            }
            TargetKind::File(file) => {
                let mut file = OpenOptions::new()
                    .append(true)
                    .open(file)?;
                file.lock()?;

                file.write_all(&buffer)?;
            }
            TargetKind::Socket(name) => {
                let mut stream = connect(name)?;

                stream.write_all(&buffer)?;
            }
        }

        Ok(())
    }
}

fn connect(name: &Name<'static>) -> Result<Stream> {
    ConnectOptions::new()
        .name(name.clone())
        .connect_sync()
}