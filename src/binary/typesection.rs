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
    fn functype<R: Read>(decoder: &mut Decoder<R>) -> DecodeResult<Func> {
        if decoder.byte()? != 0x60 {
            return Err(DecodeError::Error);
        }

        let arguments = decoder.vec(Decoder::valtype)?;
        let result = decoder.vec(Decoder::valtype)?;

        Ok(Func {
            args: arguments,
            result: result,
        })
    }

    pub fn decode<R: Read>(decoder: &mut Decoder<R>) -> DecodeResult<Self> {
        Ok(Self(decoder.vec(TypeSection::functype)?))
    }
}
