
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use crate::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    data: [u8;4]
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(bytes: [u8; 4]) -> Result<Self> {
        Ok(ChunkType::new(bytes))
    }
}

impl fmt::Display for ChunkType{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.data).expect("couldnt format data"))
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: & str) -> Result<Self> {
        let temp = ChunkType::new(s
            .as_bytes()
            .try_into()
            .expect("Couldn't convert slice into array"));
            for i in &temp.data{
                if  *i>= 65 && *i<=90{
                    continue
                } else if *i >= 97 && *i<=122 {
                    continue;
                } else {
                    // return anyhow::Error::new<'static>;
                }
            }
        Ok(temp)
    }
}

#[allow(unused)]
impl ChunkType{
    pub fn new(data: [u8;4]) -> Self {
        Self {
            data: data
        }
    }
    pub fn bytes(&self) -> [u8; 4] {
        self.data
    }
    pub fn is_valid(&self) -> bool {
        return !self.is_public() && self.is_reserved_bit_valid()
    }
    pub fn is_critical(&self) -> bool{
        if self.data[0] <= 90 {true} else{false}
    }
    pub fn is_public(&self) -> bool{
        if self.data[1] <= 90 {true} else{false}
    }
    pub fn is_reserved_bit_valid(&self) -> bool{
        if self.data[2] <= 90 {true} else{false}
    }
    pub fn is_safe_to_copy(&self) -> bool{
        if self.data[3] <= 90 {false} else{true}
    }
}