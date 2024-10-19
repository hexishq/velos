use super::{DataShredHeader, ShredCommonHeader};

pub struct ShredData {
    pub common_header: ShredCommonHeader,
    pub data_header: DataShredHeader,
    pub payload: Vec<u8>,
}
pub struct ShredCode {
    pub common_header: ShredCommonHeader,
    pub data_header: DataShredHeader,
    pub payload: Vec<u8>,
}

pub enum Shred {
    ShredCode(ShredCode),
    ShredData(ShredData),
}

//get merkle root
