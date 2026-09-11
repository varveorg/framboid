use blake3::Hash;

pub struct Claim {
    prose: String,
    focus: Vec<Hash>,
    supplementary: Vec<Hash>
}