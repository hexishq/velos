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

use bitflags::bitflags;
use solana_sdk::{clock::Slot, signature::Signature};

// LAST_SHRED_IN_SLOT also implies DATA_COMPLETE_SHRED.
// So it cannot be LAST_SHRED_IN_SLOT if not also DATA_COMPLETE_SHRED.
bitflags! {
    pub struct ShredFlags:u8 {
        const SHRED_TICK_REFERENCE_MASK = 0b0011_1111;
        const DATA_COMPLETE_SHRED       = 0b0100_0000;
        const LAST_SHRED_IN_SLOT        = 0b1100_0000;
    }
}

pub enum ShredType {
    Data = 0b1010_0101, // 165
    Code = 0b0101_1010, // 90
}

enum ShredVariant {
    LegacyCode, // 0b0101_1010
    LegacyData, // 0b1010_0101

    MerkleCode {
        proof_size: u8,
        chained: bool,
        resigned: bool,
    },
    MerkleData {
        proof_size: u8,
        chained: bool,
        resigned: bool,
    },
}

struct ShredCommonHeader {
    signature: Signature,
    shred_variant: ShredVariant,
    slot: Slot,
    shred_index: u32,
    shred_version: u16,
    fec_set_index: u32,
}

struct DataShredHeader {
    parent_offset: u16,
    data_flags: ShredFlags,
    size: u16, // common shred header + data shred header + data
}

struct CodingShredHeader {
    num_data_shreds: u16,
    num_coding_shreds: u16,
    position: u16, // [0..num_coding_shreds)
}
