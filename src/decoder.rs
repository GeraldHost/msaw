use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

use crate::types::Index;

pub type DecodeResult<T> = Result<T, DecodeError>;

pub enum DecodeError {
    Io(io::Error),
    Error,
}

impl From<io::Error> for DecodeError {
    fn from(e: io::Error) -> Self {
        DecodeError::Io(e)
    }
}

pub struct Decoder<R: Read> {
    reader: R,
}

impl<R: Read> Decoder<R> {
    pub fn new(reader: R) -> Self {
        Self { reader: reader }
    }

    pub fn byte(&mut self) -> DecodeResult<u8> {
        let mut buf = [0];
        self.reader.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    pub fn u16(&mut self) -> DecodeResult<u16> {
        let lo = self.byte()?;
        let hi = self.byte()?;
        Ok((hi as u16) << 8 | (lo as u16))
    }

    pub fn u32(&mut self) -> DecodeResult<u32> {
        let lo = self.u16()?;
        let hi = self.u16()?;
        Ok((hi as u32) << 16 | (lo as u32))
    }

    // decode leb 128 unsigned int
    // https://en.wikipedia.org/wiki/LEB128
    pub fn varunint32(&mut self) -> DecodeResult<u32> {
        let mut result = 0;
        let mut shift = 0;
        loop {
            let byte = self.byte()?;

            result |= ((byte & 0x7f) as u32) << shift;
            if byte & 0x80 == 0 {
                return Ok(result);
            }
            shift += 7;
        }
    }

    // https://webassembly.github.io/spec/core/binary/conventions.html#binary-vec
    pub fn vec<T, F>(&mut self, read_function: F) -> DecodeResult<Vec<T>>
    where
        F: Fn(&mut Decoder<R>) -> DecodeResult<T>,
    {
        let length = self.varunint32()?;
        let mut vec = Vec::with_capacity(length as usize);
        for _ in 0..length {
            vec.push(read_function(self)?);
        }
        Ok(vec)
    }

    pub fn index(&mut self) -> DecodeResult<Index> {
        self.varunint32()
    }

    pub fn valtype(&mut self) -> DecodeResult<Values> {
        match self.byte()? {
            0x7F => Ok(Values::Int(Int::I32)),
            0x7E => Ok(Values::Int(Int::I64)),
            0x7D => Ok(Values::Float(Float::F32)),
            0x7C => Ok(Values::Float(Float::F64)),
            _ => Err(DecodeError::Error),
        }
    }

    pub fn decode<T, F>(&mut self, function: F) -> DecodeResult<T>
    where
        F: Fn(&mut Decoder<R>) -> DecodeResult<T>,
    {
        function(self)
    }
}
