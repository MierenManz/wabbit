use crate::common::ConstExpr;
use crate::common::DataMode;
use crate::common::ElementMode;
use crate::common::ExportDescriptor;
use crate::common::Expr;
use crate::common::ImportDescriptor;

use crate::common::GlobalType;
use crate::common::MemType;
use crate::common::RefType;
use crate::common::TableType;
use crate::common::ValType;

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
