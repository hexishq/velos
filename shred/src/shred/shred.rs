/*
? This file is dedicated to shred, for more information, see: https://github.com/solana-foundation/specs/blob/main/p2p/shred.md

*  ** Common Header **
: The common header has size 0x53 (83 bytes).
! +--------+-----+-------------------+------------------+--------------------------------+
! | Offset | Size| Type              | Name             | Purpose                        |
! +--------+-----+-------------------+------------------+--------------------------------+
! | 0x00   | 64B | Ed25519 signature | signature        | Block producer signature       |
! | 0x40   |  1B | u8                | variant          | Shred variant                  |
! | 0x41   |  8B | u64               | slot             | Slot number                    |
! | 0x49   |  4B | u32               | shred_index      | Shred index                    |
! | 0x4d   |  2B | u16               | shred_version    | Shred version                  |
! | 0x4f   |  4B | u32               | fec_set_index    | FEC Set Index                  |
! +--------+-----+-------------------+------------------+--------------------------------+

*  ** Shred Variant Field **
: The shred variant identifies the shred type (data, code) and authentication mechanism (legacy, Merkle).
: The field is encoded as two 4-bit unsigned integers.
: The high 4-bit field is at bit range 4:8, and the low 4-bit field is at bit range 0:4.

! +------------+------------+--------------+-------------------+
! | High 4-bit | Low 4-bit  | Shred Type   | Authentication     |
! +------------+------------+--------------+-------------------+
! | 0x5        | 0xa        | Code         | Legacy             |
! | 0xa        | 0x5        | Data         | Legacy             |
! | 0x4        | Any        | Code         | Merkle             |
! | 0x8        | Any        | Data         | Merkle             |
! +------------+------------+--------------+-------------------+

*  ** Data Shred Header **
! +--------+-----+-------+----------------+--------------------------------+
! | Offset | Size| Type  | Name           | Purpose                        |
! +--------+-----+-------+----------------+--------------------------------+
! | 0x53   | 2B  | u16   | parent_offset  | Slot distance to parent block  |
! | 0x55   | 1B  | u8    | data_flags     | Data Flags                     |
! | 0x56   | 2B  | u16   | size           | Total Size                     |
! +--------+-----+-------+----------------+--------------------------------+


*  ** Code Shred Header **
! +--------+-----+-------+--------------------+-----------------------------------------+
! | Offset | Size| Type  | Name               | Purpose                                 |
! +--------+-----+-------+--------------------+-----------------------------------------+
! | 0x53   | 2B  | u16   | num_data_shreds    | Number of data shreds                   |
! | 0x55   | 2B  | u16   | num_coding_shreds  | Number of coding shreds                 |
! | 0x57   | 2B  | u16   | position           | Position of this shred in FEC set       |
! +--------+-----+-------+--------------------+-----------------------------------------+

*  ** Shred Packet Size **
: The maximum shred packet size is determined based on the IPv6 minimum link MTU.

! Max size for shred packet is 1228 bytes (Legacy) or 1203 bytes (Merkle).
*/

use std::error::Error;

use crate::shred::{shred_code::ShredCode, shred_data::ShredData};
use bitflags::bitflags;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use solana_sdk::{
    clock::Slot,
    hash::{hashv, Hash},
    pubkey::Pubkey,
    signature::Signature,
};
mod layout;
mod legacy;
mod merkle;
mod shred_code;
mod shred_data;
mod verify_shred;

// LAST_SHRED_IN_SLOT also implies DATA_COMPLETE_SHRED.
// So it cannot be LAST_SHRED_IN_SLOT if not also DATA_COMPLETE_SHRED.
bitflags! {
    pub struct ShredFlags:u8 {
        const SHRED_TICK_REFERENCE_MASK = 0b0011_1111;
        const DATA_COMPLETE_SHRED       = 0b0100_0000;
        const LAST_SHRED_IN_SLOT        = 0b1100_0000;
    }
}
#[repr(u8)]
#[cfg_attr(feature = "frozen-abi", derive(AbiExample, AbiEnumVisitor))]
#[derive(
    PartialEq, Eq, Debug, Clone, Copy, Hash, TryFromPrimitive, IntoPrimitive, Serealize, Deserealize,
)]
pub enum ShredType {
    Data = 0b1010_0101, // 165
    Code = 0b0101_1010, // 90
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum ShredVariant {
    LegacyCode, // 0b0101_1010
    LegacyData, // 0b1010_0101
    // proof_size is the number of Merkle proof entries, and is encoded in the
    // lowest 4 bits of the binary representation. The first 4 bits identify
    // the shred variant:
    //   0b0100_????  MerkleCode
    //   0b0110_????  MerkleCode chained
    //   0b0111_????  MerkleCode chained resigned
    //   0b1000_????  MerkleData
    //   0b1001_????  MerkleData chained
    //   0b1011_????  MerkleData chained resigned
    MerkleCode {
        proof_size: u8,
        chained: bool,
        resigned: bool,
    }, // 0b01??_????
    MerkleData {
        proof_size: u8,
        chained: bool,
        resigned: bool,
    }, // 0b10??_????
}

impl ShredVariant {
    pub fn from_u8(bytes: u8) -> Result<ShredVariant, ()> {
        match bytes {
            0b0101_1010 => return Ok(ShredVariant::LegacyCode),
            0b1010_0101 => return Ok(ShredVariant::LegacyData),
            v if (v & 0b1111_0000) == 0b0110_0000 => {
                let proof_size = v & 0b0000_1111;
                let chained = true;
                let resigned = false;
                Ok(ShredVariant::MerkleCode {
                    proof_size,
                    chained,
                    resigned,
                })
            }

            v if (v & 0b1111_0000) == 0b0111_0000 => {
                let proof_size = v & 0b0000_1111;
                let chained = true;
                let resigned = true;
                Ok(ShredVariant::MerkleCode {
                    proof_size,
                    chained,
                    resigned,
                })
            }

            v if (v & 0b1111_0000) == 0b1000_0000 => {
                let proof_size = v & 0b0000_1111;
                let chained = false;
                let resigned = false;
                Ok(ShredVariant::MerkleData {
                    proof_size,
                    chained,
                    resigned,
                })
            }

            v if (v & 0b1111_0000) == 0b1001_0000 => {
                let proof_size = v & 0b0000_1111;
                let chained = true;
                let resigned = false;
                Ok(ShredVariant::MerkleData {
                    proof_size,
                    chained,
                    resigned,
                })
            }

            v if (v & 0b1111_0000) == 0b1011_0000 => {
                let proof_size = v & 0b0000_1111;
                let chained = true;
                let resigned = true;
                Ok(ShredVariant::MerkleData {
                    proof_size,
                    chained,
                    resigned,
                })
            }
            _ => Err(()),
        }
    }
}

pub struct ShredCommonHeader {
    signature: Signature,
    shred_variant: ShredVariant,
    slot: Slot,
    shred_index: u32,
    shred_version: u16,
    fec_set_index: u32,
}

pub struct DataShredHeader {
    parent_offset: u16,
    data_flags: ShredFlags,
    size: u16, // common shred header + data shred header + data
}

pub struct CodingShredHeader {
    num_data_shreds: u16,
    num_coding_shreds: u16,
    position: u16, // [0..num_coding_shreds)
}

pub enum Shred {
    ShredCode(ShredCode),
    ShredData(ShredData),
}

pub enum SignedData<'a> {
    Chunk(&'a [u8]), // chunk of payload past signature
    MerkleRoot(Hash),
}

impl<'a> AsRef<[u8]> for SignedData<'a> {
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::Chunk(chunk) => chunk,
            Self::MerkleRoot(root) => root.as_ref(),
        }
    }
}

// A tuple which identifies if a shred should it exist in the shred queue.
pub struct ShredId(Slot, /*shred index:*/ u32, ShredType);

impl ShredId {
    pub fn new(slot: Slot, shred_index: u32, shred_type: ShredType) -> ShredId {
        ShredId(slot, shred_index, shred_type)
    }

    pub fn slot(&self) -> Slot {
        self.0
    }

    pub fn unpack(&self) -> (Slot, /*shred index:*/ u32, ShredType) {
        (self.0, self.1, self.2)
    }

    pub fn seed(&self, leader: &Pubkey) -> [u8; 32] {
        let ShredId(slot, index, shred_type) = self;
        hashv(&[
            &slot.to_le_bytes(),
            &u8::from(*shred_type).to_le_bytes(),
            &index.to_le_bytes(),
            AsRef::<[u8]>::as_ref(leader),
        ])
        .to_bytes()
    }
}

// A tuple that identifies erasure coding set that the shred belongs to

pub struct ErasureSetId(Slot, /*fec_set_index*/ u32);

impl ErasureSetId {
    pub fn new(slot: Slot, fec_set_index: u32) -> Self {
        Self(slot, fec_set_index)
    }

    pub fn slot(&self) -> Slot {
        self.0
    }

    // Storage key for ErasureMeta and MerkleRootMeta in blockstore
    // Note: ErasureMeta column uses u64 so this will need to be typecasted
    pub fn store_key(&self) -> (Slot, /*fec_set_index*/ u32) {
        (self.0, self.1)
    }
}
