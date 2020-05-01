use crate::decoder::{DecodeError, DecodeResult, Decoder};
use crate::types::Index;
use std::io::Read;

#[derive(Debug)]
pub enum ExportDesc {
    Func(Index),
    Table(Index),
    Mem(Index),
    Global(Index),
}

#[derive(Debug)]
pub struct Export {
    pub name: String,
    pub description: ExportDesc,
}

pub struct ExportSection(Vec<Export>);

impl ExportSection {
    /// https://webassembly.github.io/spec/core/binary/values.html#binary-name
    fn name<R: Read>(decoder: &mut Decoder<R>) -> DecodeResult<String> {
        let vec = decoder.vec(Decoder::byte)?;
        match String::from_utf8(vec) {
            Ok(string) => Ok(string),
            Err(_) => Err(DecodeError::Error),
        }
    }

    fn description<R: Read>(decoder: &mut Decoder<R>) -> DecodeResult<ExportDesc> {
        match decoder.byte()? {
            0x00 => Ok(ExportDesc::Func(decoder.index()?)),
            0x01 => Ok(ExportDesc::Table(decoder.index()?)),
            0x02 => Ok(ExportDesc::Mem(decoder.index()?)),
            0x03 => Ok(ExportDesc::Global(decoder.index()?)),
            _ => Err(DecodeError::Error),
        }
    }

    fn export<R: Read>(decoder: &mut Decoder<R>) -> DecodeResult<Export> {
        let name = decoder.decode(ExportSection::name)?;
        let description = decoder.decode(ExportSection::description)?;
        Ok(Export {
            name: name,
            description: description,
        })
    }

    pub fn decode<R: Read>(decoder: &mut Decoder<R>) -> DecodeResult<Self> {
        Ok(Self(decoder.vec(ExportSection::export)?))
    }
}
