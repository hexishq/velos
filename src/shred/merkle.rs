use super::{DataShredHeader, ShredCommonHeader};

pub struct ShredData {
    common_header: ShredCommonHeader,
    data_header: DataShredHeader,
    payload: Vec<u8>,
}
pub struct ShredCode {
    common_header: ShredCommonHeader,
    data_header: DataShredHeader,
    payload: Vec<u8>,
}

pub enum Shred {
    ShredCode(ShredCode),
    ShredData(ShredData),
}
