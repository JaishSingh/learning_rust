use std::fmt;

use crate::chunk_type::ChunkType;
use crate::{Error, Result};
pub struct Chunk{
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        // println!("{:?}", &value);
        let chunk_type = ChunkType::new(value[4..8].try_into().expect("error in creating ChunkType"));
        let data = value[8..value.len()-4].to_vec();
        return Ok(Chunk::new(chunk_type, data));
    }
}

impl fmt::Display for Chunk{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data size: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}")?;
        Ok(())
    }
}

#[allow(unused)]
impl Chunk{
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let length = data.len() as u32;
        let hashing_data = [chunk_type.bytes().as_slice(), data.as_slice()].concat();
        let crc = crc::crc32::checksum_ieee(&hashing_data);
        Self { 
            length,
            chunk_type,
            data,
            crc
        }
    }
    pub fn length(&self) -> u32 {
        self.length
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    pub fn crc(&self) -> u32 {
        self.crc
    }
    pub fn data_as_string(&self) -> Result<String> {
        Ok(String::from_utf8_lossy(&self.data).to_string())
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        [
            self.length.to_be_bytes().as_ref(),
            &self.chunk_type.bytes(),
            self.data.as_slice(),
            self.crc.to_be_bytes().as_ref(),
        ]
        .concat()
    }
}