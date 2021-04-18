#[derive(Debug)]
pub struct HexData(pub Vec<u8>);

impl std::str::FromStr for HexData {
    type Err = hex::FromHexError;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        if 1 < s.len() && s.starts_with("0x") {
            s = &s[2..];
        }
        hex::decode(s).map(HexData)
    }
}
