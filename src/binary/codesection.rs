use std::io::Read;
use crate::decoder::{DecodeError, DecodeResult, Decoder};

type Local = (u32, u32);

struct Code {
    locals: Vec<Local>,
    body: _,
}

pub struct CodeSection(Vec<Code>);

impl CodeSection {

  // read locals
  fn locals<R: Read>(decoder: &mut Decoder<R>) -> DecodeResult<Local> {
    let count = decoder.varuint32()?;
	let valtype = decoder.valtype()?;
	Ok((count, valtype))
  }

  fn func<R: Read>(decoder: &mut Decoder<R>) -> DecodeResult<Code> {
    let body_size = decoder.varuint32()?;
    let locals = decoder.vec(CodeSection::locals)?;
    let body = decoder.decode(CodeSection::body)?;
    Ok(Code {
        locals: locals,
        body: body,
    })    
  }

  // read code section
  // read instructions
  fn body() {}

  pub fn decode<R: Read>(decoder: &mut Decoder<R>) -> DecodeResult<Self> {
    decoder.vec(CodeSection::func)
  }
}
