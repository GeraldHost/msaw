use crate::decoder::{DecodeError, DecodeResult, Decoder};
use crate::types::*;
use std::io::Read;

#[derive(Debug)]
pub struct Func {
    pub args: Vec<Values>,
    pub result: Vec<Values>,
}

pub struct TypeSection(Vec<Func>);

impl TypeSection {
    // https://webassembly.github.io/spec/core/binary/types.html#binary-valtype
    // https://webassembly.github.io/spec/core/binary/types.html#binary-resulttype
    fn valtype<R: Read>(decoder: &mut Decoder<R>) -> DecodeResult<Values> {
        match decoder.byte()? {
            0x7F => Ok(Values::Int(Int::I32)),
            0x7E => Ok(Values::Int(Int::I64)),
            0x7D => Ok(Values::Float(Float::F32)),
            0x7C => Ok(Values::Float(Float::F64)),
            _ => Err(DecodeError::Error),
        }
    }

    fn functype<R: Read>(decoder: &mut Decoder<R>) -> DecodeResult<Func> {
        if decoder.byte()? != 0x60 {
            return Err(DecodeError::Error);
        }

        let arguments = decoder.vec(TypeSection::valtype)?;
        let result = decoder.vec(TypeSection::valtype)?;

        Ok(Func {
            args: arguments,
            result: result,
        })
    }

    pub fn decode<R: Read>(decoder: &mut Decoder<R>) -> DecodeResult<Self> {
        Ok(Self(decoder.vec(TypeSection::functype)?))
    }
}
