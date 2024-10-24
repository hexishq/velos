use super::{CodingShredHeader, DataShredHeader, ShredCommonHeader};

pub struct ShredData {
    pub common_header: ShredCommonHeader,
    pub data_header: DataShredHeader,
    pub payload: Vec<u8>,
}

pub struct ShredCode {
    pub common_header: ShredCommonHeader,
    pub coding_header: CodingShredHeader,
    pub payload: Vec<u8>,
}
