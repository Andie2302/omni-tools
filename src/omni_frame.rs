use crate::omni_data::OmniData;
use crate::varint::VarInt;

pub struct OmniFrameHeader {
    pub block_type: Vec<VarInt>,
}

pub struct OmniFrameContent {
    pub data: OmniData,
} 

pub struct OmniFrameFooter {
    pub data: OmniData,
}

pub struct OmniFrame {
    pub header: OmniFrameHeader,
    pub content: OmniFrameContent,
    pub footer: Option<OmniFrameFooter>,
}

impl OmniFrame {
    pub fn new(header: OmniFrameHeader, content: OmniFrameContent, footer: Option<OmniFrameFooter>) -> Self {
        Self { header, content, footer }
    }
}

impl OmniFrameHeader {
    pub fn new(ids: impl Into<Vec<VarInt>>) -> Self {
        Self { block_type: ids.into() }
    }
}

impl OmniFrameContent {
    pub fn new(data: OmniData) -> Self {
        Self { data }
    }
}

impl OmniFrameFooter {
    pub fn new(data: OmniData) -> Self {
        Self { data }
    }
}
