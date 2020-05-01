#[derive(Debug)]
pub enum Int {
    I32,
    I64,
}

#[derive(Debug)]
pub enum Float {
    F32,
    F64,
}

#[derive(Debug)]
pub enum Values {
    Float(Float),
    Int(Int),
}

pub type Index = u32;

pub struct Code {
    pub locals: Vec<(u32, Values)>,
    /// not sure what this should be at this point
    pub body: u32,
}
