pub mod error;

use std::net::ToSocketAddrs;

use url::Url;

use crate::target::endpoint::error::EndpointError;

/// A `Target` that transports over the internet.
pub struct Endpoint(pub(super) EndpointKind);
pub(super) enum EndpointKind {
    Http(Url),
    Tcp(String),
    Udp(String)
}

impl Endpoint {
    /// Validates URL and converts into `Url`.
    pub fn http(url: String) -> Result<Self, EndpointError> {
        let url = Url::parse(&url).map_err(|_| EndpointError::InvalidUrl(url))?;

        Ok(Endpoint(EndpointKind::Http(url)))
    }

    /// Validates address.
    pub fn tcp(addr: String) -> Result<Self, EndpointError> {
        let Ok(mut resolved) = addr.to_socket_addrs() else {
            return Err(EndpointError::InvalidAddress(addr));
        };

        if resolved.next().is_none() {
            return Err(EndpointError::InvalidAddress(addr));
        }

        Ok(Endpoint(EndpointKind::Tcp(addr)))
    }

    /// Validates address.
    pub fn udp(addr: String) -> Result<Self, EndpointError> {
        let Ok(mut resolved) = addr.to_socket_addrs() else {
            return Err(EndpointError::InvalidAddress(addr));
        };

        if resolved.next().is_none() {
            return Err(EndpointError::InvalidAddress(addr));
        }
        
        Ok(Endpoint(EndpointKind::Udp(addr)))
    }
}

