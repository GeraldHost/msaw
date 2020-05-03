use std::io;
use std::io::Read;

use crate::decoder::{DecodeError, DecodeResult, Decoder};
use crate::types::Index;

use crate::exportsection::ExportSection;
use crate::functionsection::FunctionSection;
use crate::typesection::TypeSection;
use crate::codesection::CodeSection;

const WASM_MAGIC: u32 = 0x6d736100;
const WASM_VERSION: u32 = 1;

type ModuleHeader = (u32, u32);

pub enum Section {
    Type(TypeSection),
    Function(FunctionSection),
    Export(ExportSection),
    Code(CodeSection),
}

pub struct Module {
    pub header: ModuleHeader,
    sections: Vec<Section>,
}

impl Module {
    // read wasm module header which consits of two u32
    // the magic number aka \0asm and the version (currently 1)
    fn header<R: Read>(decoder: &mut Decoder<R>) -> DecodeResult<ModuleHeader> {
        let magic_number = decoder.u32()?;
        let version = decoder.u32()?;

        if magic_number != WASM_MAGIC {
            return Err(DecodeError::Error);
        }

        if version != WASM_VERSION {
            return Err(DecodeError::Error);
        }

        Ok((magic_number, version))
    }

    fn sections<R: Read>(decoder: &mut Decoder<R>) -> DecodeResult<Vec<Section>> {
        loop {
            match decoder.byte() {
                Err(DecodeError::Io(ref e)) if e.kind() == io::ErrorKind::UnexpectedEof => break,
                Ok(id) => {
                    let size = decoder.varunint32()?;
                    match id {
                        1 => Section::Type(TypeSection::decode(decoder)?),
                        2 => panic!("2"),
                        3 => Section::Function(FunctionSection::decode(decoder)?),
                        4 => panic!("4"),
                        5 => panic!("5"),
                        6 => panic!("6"),
                        7 => Section::Export(ExportSection::decode(decoder)?),
                        8 => panic!("8"),
                        9 => panic!("9"),
                        10 => Section::Code(CodeSection::decode(decoder)?), 
                        11 => panic!("11"),
                        _ => return Err(DecodeError::Error),
                    }
                }
                Err(error) => return Err(DecodeError::Error),
            };
        }
        Ok(vec![])
    }

    pub fn decode<R: Read>(decoder: &mut Decoder<R>) -> DecodeResult<Module> {
        let header = decoder.decode(Module::header)?;
        let sections = decoder.decode(Module::sections)?;
        let module = Module {
            header: header,
            sections: sections,
        };
        Ok(module)
    }
}
