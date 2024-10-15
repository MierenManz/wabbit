use crate::descriptors::ConstExpr;
use crate::descriptors::DataMode;
use crate::descriptors::ElementMode;
use crate::descriptors::ExportDescriptor;
use crate::descriptors::Expr;
use crate::descriptors::ImportDescriptor;

use crate::types::GlobalType;
use crate::types::MemType;
use crate::types::RefType;
use crate::types::TableType;
use crate::types::ValType;

pub struct LirImport {
    module: String,
    name: String,
    descriptor: ImportDescriptor,
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

pub struct LirElement {
    ref_type: RefType,
    init: Vec<ConstExpr>,
    mode: ElementMode,
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
