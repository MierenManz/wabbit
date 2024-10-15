#[derive(Clone, Copy)]
pub enum NumType {
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C,
}

#[derive(Clone, Copy)]
pub enum VecType {
    V128 = 0x7B,
}

#[derive(Clone, Copy)]
pub enum RefType {
    Func = 0x70,
    External = 0x6F,
}

#[derive(Clone, Copy)]
pub enum ValType {
    Num(NumType),
    Vec(VecType),
    Ref(RefType),
}

impl From<NumType> for ValType {
    fn from(value: NumType) -> Self {
        Self::Num(value)
    }
}

pub type ResultType = Vec<ValType>;

pub struct FuncType {
    values: ResultType,
    param_count: u32,
}

impl FuncType {
    // TODO: Add error type
    pub fn new(params: &[ValType], result: &[ValType]) -> Result<Self, ()> {
        if params.len() > u32::MAX as usize || result.len() > u32::MAX as usize {
            return Err(());
        }

        Ok(Self::new_unchecked(params, result))
    }

    pub fn new_unchecked(params: &[ValType], result: &[ValType]) -> Self {
        let mut x = Vec::with_capacity(params.len() + result.len());
        x.extend_from_slice(params);
        x.extend_from_slice(result);

        Self {
            values: x,
            param_count: params.len() as u32,
        }
    }

    pub(crate) fn params(&self) -> &[ValType] {
        &self.values[..self.param_count as usize]
    }

    pub(crate) fn result(&self) -> &[ValType] {
        &self.values[self.param_count as usize..]
    }
}

#[derive(Clone, Copy)]
pub struct Limits {
    min: u32,
    max: Option<u32>,
}

impl Limits {
    pub fn new(min: u32, max: Option<u32>) -> Self {
        Self { min, max }
    }

    pub fn min(&self) -> u32 {
        self.min
    }

    pub fn max(&self) -> Option<u32> {
        self.max
    }
}

pub type MemType = Limits;

pub struct TableType {
    ref_type: RefType,
    limits: Limits,
}

impl TableType {
    pub fn new(ref_type: RefType, limits: Limits) -> Self {
        Self { ref_type, limits }
    }

    pub fn ref_type(&self) -> RefType {
        self.ref_type
    }

    pub fn limits(&self) -> Limits {
        self.limits
    }
}

pub struct GlobalType {
    val_type: ValType,
    mutable: bool,
}
