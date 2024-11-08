use {
    serde::{Deserialize, Serialize},
    solana_sdk::pubkey::Pubkey,
    std::{
        net::{IpAddr, Ipv4Addr, SocketAddr},
        time::{SystemTime, UNIX_EPOCH},
    },
};

const SOCKET_CACHE_SIZE: usize = 12;
const SOCKET_ADDR_UNSPECIFIED: SocketAddr =
    SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), /*port:*/ 0u16);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ContactInfo {
    pubkey: Pubkey,
    wallclock: u64,
    outset: u64,
    shred_version: u16,
    version: solana_version::Version,
    addrs: Vec<IpAddr>,
    sockets: Vec<SocketEntry>,
    extensions: Vec<Extension>,
    cache: [SocketAddr; SOCKET_CACHE_SIZE],
}

impl ContactInfo {
    pub fn pubkey(&self) -> &Pubkey {
        &self.pubkey
    }

    pub fn sockets(&self) -> &Vec<SocketEntry> {
        &self.sockets
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
enum Extension {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SocketEntry {
    key: u8,
    index: u8,
    offset: u16,
}

fn get_timestamp() -> u64 {
    let now = SystemTime::now();
    let elapsed = now.duration_since(UNIX_EPOCH).unwrap();
    u64::try_from(elapsed.as_micros()).unwrap()
}

impl ContactInfo {
    pub fn new(pubkey: Pubkey, wallclock: u64, shred_version: u16, gossip: SocketAddr) -> Self {
        let mut vec_socket = vec![];

        let socket_entry = SocketEntry {
            key: 0,
            index: 0,
            offset: gossip.port(),
        };

        vec_socket.push(socket_entry);

        Self {
            pubkey,
            wallclock,
            outset: get_timestamp(),
            shred_version,
            version: solana_version::Version::default(),
            addrs: Vec::<IpAddr>::default(),
            sockets: vec_socket,
            extensions: Vec::<Extension>::default(),
            cache: [SOCKET_ADDR_UNSPECIFIED; SOCKET_CACHE_SIZE],
        }
    }
}
