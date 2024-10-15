use crate::indices::FuncIdx;
use crate::indices::GlobalIdx;
use crate::indices::MemIdx;
use crate::indices::TableIdx;
use crate::indices::TypeIdx;
use crate::types::GlobalType;
use crate::types::MemType;
use crate::types::RefType;
use crate::types::TableType;
use crate::types::ValType;

// TODO: Add expression type
type Expr = ();
type ConstExpr = ();

pub struct LirImport {
    module: String,
    name: String,
    descriptor: ImportDescriptor,
}

pub enum ImportDescriptor {
    Func(TypeIdx),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}

pub type LirTable = TableType;
pub type LirMemory = MemType;

pub struct LirGlobal {
    global_type: GlobalType,
    init: Expr,
}

pub struct LirExport {
    name: String,
    descriptor: ExportDescriptor,
}

#[repr(u8)]
pub enum ExportDescriptor {
    Func(FuncIdx) = 0x00,
    Table(TableIdx) = 0x01,
    Memory(MemIdx) = 0x02,
    Global(GlobalIdx) = 0x03,
}

pub struct LirElement {
    ref_type: RefType,
    init: Vec<ConstExpr>,
    mode: ElementMode,
}

pub enum ElementMode {
    Passive,
    Active { table: TableIdx, offset: ConstExpr },
    Declarative,
}

pub struct LirCode {
    size: u32,
    code: LirFunc,
}

pub struct LirFunc {
    locals: Vec<LirLocal>,
    code: Expr,
}

pub struct LirLocal {
    count: u32,
    val_type: ValType,
}

pub struct LirData {
    init: Vec<u8>,
    mode: DataMode,
}

pub enum DataMode {
    Passive,
    Active { memory: MemIdx, offset: ConstExpr },
}
