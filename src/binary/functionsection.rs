use crate::decoder::{DecodeError, DecodeResult, Decoder};
use crate::types::Index;
use std::io::Read;

pub struct FunctionSection(Vec<Index>);

impl FunctionSection {
    pub fn decode<R: Read>(decoder: &mut Decoder<R>) -> DecodeResult<Self> {
        Ok(Self(decoder.vec(Decoder::index)?))
    }
}
