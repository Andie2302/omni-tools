
pub struct OmniByte
{
    pub data: Vec<u8>,
}
impl OmniByte
{
    pub fn new() -> Self
    {
        Self {
            data: vec![],
        }
    }
}


pub struct OmniNumber
{
    pub data_type: Vec<u8>,
    pub d1: Vec<u8>,
    pub d2: Vec<u8>,
    pub d3: Vec<u8>,
}

impl OmniNumber
{
    pub fn new() -> Self
    {
        Self {
            data_type: vec![],
            d1: vec![],
            d2: vec![],
            d3: vec![],
        }
    }
}