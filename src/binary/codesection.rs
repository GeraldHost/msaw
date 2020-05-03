use std::io::Read;
use crate::decoder::{DecodeError, DecodeResult, Decoder};
use crate::types::*;

type Local = (u32, Values);

struct Code {
    locals: Vec<Local>,
    body: u32,
}

pub struct CodeSection(Vec<Code>);

impl CodeSection {

  // read locals
  fn locals<R: Read>(decoder: &mut Decoder<R>) -> DecodeResult<Local> {
    let count = decoder.varunint32()?;
	let valtype = decoder.valtype()?;
	Ok((count, valtype))
  }

  fn func<R: Read>(decoder: &mut Decoder<R>) -> DecodeResult<Code> {
    let body_size = decoder.varunint32()?;
    let locals = decoder.vec(CodeSection::locals)?;
    let body = decoder.decode(CodeSection::body)?;
    Ok(Code {
        locals: locals,
        body: body,
    })    
  }

  // read code section
  // read instructions
  fn body<R: Read>(decoder: &mut Decoder<R>) -> DecodeResult<u32> {
    loop {
        match decoder.byte()? {
            0x0b => {
                panic!("lame");
                return Ok(1);
            },
            b => println!("{:?}", format!("{:x}", b)),
        }
    }
    Ok(0)
  }

  pub fn decode<R: Read>(decoder: &mut Decoder<R>) -> DecodeResult<Self> {
    Ok(Self(decoder.vec(CodeSection::func)?))
  }
}
