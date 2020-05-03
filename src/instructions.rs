use std::io::Read;
use crate::decoder::{Decoder, DecodeResult, DecodeError};

pub struct Instructions(Vec<Instruction>);

impl Instructions {
    pub fn decode<R: Read>(decoder: &mut Decoder<R>) -> DecodeResult<Vec<Instruction>> {
        use self::Instruction::*;
        let mut vec = Vec::new();
        // lets start by decoding just the op codes required for the add.wasm
        // file. Just as an initial POC.
        loop {
            let op = match decoder.byte()? {
                0x00 => Unreachable,
                0x01 => Nop,
                0x20 => GetLocal(decoder.index()?),
                0x6a => I32Add,
                0x0b => return Ok(vec),
                _ => return Err(DecodeError::Error),
            };
            vec.push(op);
        }
    }
}

pub enum ValueType {
	/// 32-bit signed integer
	I32,
	/// 64-bit signed integer
	I64,
	/// 32-bit float
	F32,
	/// 64-bit float
	F64,
}

pub enum BlockType {
	/// Value-type specified block type
	Value(ValueType),
	/// No specified block type
	NoResult,
}

pub struct BrTableData {
	pub table: Box<[u32]>,
	pub default: u32,
}

pub enum Instruction {
	Unreachable,
	Nop,
	Block(BlockType),
	Loop(BlockType),
	If(BlockType),
	Else,
	End,
	Br(u32),
	BrIf(u32),
	BrTable(Box<BrTableData>),
	Return,

	Call(u32),
	CallIndirect(u32, u8),

	Drop,
	Select,

	GetLocal(u32),
	SetLocal(u32),
	TeeLocal(u32),
	GetGlobal(u32),
	SetGlobal(u32),

	// All store/load instructions operate with 'memory immediates'
	// which represented here as (flag, offset) tuple
	I32Load(u32, u32),
	I64Load(u32, u32),
	F32Load(u32, u32),
	F64Load(u32, u32),
	I32Load8S(u32, u32),
	I32Load8U(u32, u32),
	I32Load16S(u32, u32),
	I32Load16U(u32, u32),
	I64Load8S(u32, u32),
	I64Load8U(u32, u32),
	I64Load16S(u32, u32),
	I64Load16U(u32, u32),
	I64Load32S(u32, u32),
	I64Load32U(u32, u32),
	I32Store(u32, u32),
	I64Store(u32, u32),
	F32Store(u32, u32),
	F64Store(u32, u32),
	I32Store8(u32, u32),
	I32Store16(u32, u32),
	I64Store8(u32, u32),
	I64Store16(u32, u32),
	I64Store32(u32, u32),

	CurrentMemory(u8),
	GrowMemory(u8),

	I32Const(i32),
	I64Const(i64),
	F32Const(u32),
	F6Const(u64),

	I32Eqz,
	I32Eq,
	I32Ne,
	I32LtS,
	I32LtU,
	I32GtS,
	I32GtU,
	I32LeS,
	I32LeU,
	I32GeS,
	I32GeU,

	I64Eqz,
	I64Eq,
	I64Ne,
	I64LtS,
	I64LtU,
	I64GtS,
	I64GtU,
	I64LeS,
	I64LeU,
	I64GeS,
	I64GeU,

	F32Eq,
	F32Ne,
	F32Lt,
	F32Gt,
	F32Le,
	F32Ge,

	F64Eq,
	F64Ne,
	F64Lt,
	F64Gt,
	F64Le,
	F64Ge,

	I32Clz,
	I32Ctz,
	I32Popcnt,
	I32Add,
	I32Sub,
	I32Mul,
	I32DivS,
	I32DivU,
	I32RemS,
	I32RemU,
	I32And,
	I32Or,
	I32Xor,
	I32Shl,
	I32ShrS,
	I32ShrU,
	I32Rotl,
	I32Rotr,

	I64Clz,
	I64Ctz,
	I64Popcnt,
	I64Add,
	I64Sub,
	I64Mul,
	I64DivS,
	I64DivU,
	I64RemS,
	I64RemU,
	I64And,
	I64Or,
	I64Xor,
	I64Shl,
	I64ShrS,
	I64ShrU,
	I64Rotl,
	I64Rotr,
	F32Abs,
	F32Neg,
	F32Ceil,
	F32Floor,
	F32Trunc,
	F32Nearest,
	F32Sqrt,
	F32Add,
	F32Sub,
	F32Mul,
	F32Div,
	F32Min,
	F32Max,
	F32Copysign,
	F64Abs,
	F64Neg,
	F64Ceil,
	F64Floor,
	F64Trunc,
	F64Nearest,
	F64Sqrt,
	F64Add,
	F64Sub,
	F64Mul,
	F64Div,
	F64Min,
	F64Max,
	F64Copysign,

	I32WrapI64,
	I32TruncSF32,
	I32TruncUF32,
	I32TruncSF64,
	I32TruncUF64,
	I64ExtendSI32,
	I64ExtendUI32,
	I64TruncSF32,
	I64TruncUF32,
	I64TruncSF64,
	I64TruncUF64,
	F32ConvertSI32,
	F32ConvertUI32,
	F32ConvertSI64,
	F32ConvertUI64,
	F32DemoteF64,
	F64ConvertSI32,
	F64ConvertUI32,
	F64ConvertSI64,
	F64ConvertUI64,
	F64PromoteF32,

	I32ReinterpretF32,
	I64ReinterpretF64,
	F32ReinterpretI32,
	F64ReinterpretI64,
}
