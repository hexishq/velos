use std::net::IpAddr;

use bitflags::bitflags;
use solana_sdk::clock::Slot;

use super::layout;

const PACKET_DATA_SIZE: usize = 1280 - 40 - 8;
const SIZE_OF_NONCE: usize = 4;

pub struct Packet {
    pub data: [u8; PACKET_DATA_SIZE],
    pub meta: Meta,
}

pub struct Meta {
    pub size: usize,
    pub addr: IpAddr,
    pub port: u16,
    pub flags: PacketFlags,
}

bitflags! {
    pub struct PacketFlags: u8 {
        const DISCARD        = 0b0000_0001;
        const FORWARDED      = 0b0000_0010;
        const REPAIR         = 0b0000_0100;
        const SIMPLE_VOTE_TX = 0b0000_1000;
        const TRACER_PACKET  = 0b0001_0000;
        // Previously used - this can now be re-used for something else.
        const UNUSED = 0b0010_0000;
        /// For tracking performance
        const PERF_TRACK_PACKET  = 0b0100_0000;
        /// For marking packets from staked nodes
        const FROM_STAKED_NODE = 0b1000_0000;
    }
}

enum VerifyShredErros {
    InvalidShredSize,
    SlotMissing,
    SignatureMissing,
}

fn verify_shred(packet: Packet) -> Result<(), VerifyShredErros> {
    let shred = match packet.get_shred() {
        Some(shred) => shred,
        None => return Err(VerifyShredErros::InvalidShredSize),
    };

    let slot: Slot = match layout::get_slot(&shred) {
        Some(slot) => slot,
        None => return Err(VerifyShredErros::SlotMissing),
    };

    let signature = match layout::get_signature(&shred) {
        Some(signature) => signature,
        None => return Err(VerifyShredErros::SignatureMissing),
    };

    Ok(())
}

impl Packet {
    fn get_packet_size(&self) -> usize {
        if self.meta.flags.contains(PacketFlags::REPAIR) {
            return PACKET_DATA_SIZE.saturating_sub(SIZE_OF_NONCE);
        }
        return PACKET_DATA_SIZE;
    }

    fn get_shred(&self) -> Option<Vec<u8>> {
        let shred_size = self.get_packet_size();
        if shred_size == 0 {
            return None;
        }
        Some(self.data[0..shred_size].to_vec())
    }
}
