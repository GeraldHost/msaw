use std::io::Read;
use crate::decoder::{DecodeError, DecodeResult, Decoder};

pub struct CodeSection(Vec<Code>);

impl CodeSection {

  // read locals
  fn locals() {}

  fn func<R: Read>(decoder: &mut Decoder<R>) {
    let body_size = decoder.varuint32()?;
    let locals = decoder.vec(CodeSection::locals)?;
    let code = decoder.decode(CodeSection::code)?;
    
  }

  // read code section
  // read instructions
  fn code() {}

  pub fn decode<R: Read>(decoder: &mut Decoder<R>) -> DecodeResult<Self> {
    decoder.vec(CodeSection::func)
  }
}