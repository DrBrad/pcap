#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum UdpTypes {
    Dhcp,
    Dns,
    Quick,
    uTp,
    BitTorrent,
    Unknown
}

impl UdpTypes {

    pub fn to_string(&self) -> String {
        match self {
            Self::Dhcp => "DHCP",
            _ => "UDP"
        }.to_string()
    }
}
