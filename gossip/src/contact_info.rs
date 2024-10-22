use std::net::{IpAddr, SocketAddr};

use solana_sdk::pubkey::Pubkey;

const SOCKET_CACHE_SIZE: usize = 12;
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

enum Extension {}

struct SocketEntry {
    key: u8,
    index: u8,
    offset: u16,
}
