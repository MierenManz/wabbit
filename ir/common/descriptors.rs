use crate::common::FuncIdx;
use crate::common::GlobalIdx;
use crate::common::MemIdx;
use crate::common::TableIdx;
use crate::common::TypeIdx;
use crate::common::GlobalType;
use crate::common::MemType;
use crate::common::TableType;
use crate::common::Instruction;
use crate::common::ConstInstruction;

pub(crate) type Expr = Vec<Instruction>;
pub(crate) type ConstExpr = Vec<ConstInstruction>;

#[repr(u8)]
pub(crate) enum ImportDescriptor {
    Func(TypeIdx) = 0x00,
    Table(TableType) = 0x01,
    Mem(MemType) = 0x02,
    Global(GlobalType) = 0x03,
}

#[repr(u8)]
pub(crate) enum ExportDescriptor {
    Func(FuncIdx) = 0x00,
    Table(TableIdx) = 0x01,
    Memory(MemIdx) = 0x02,
    Global(GlobalIdx) = 0x03,
}

pub(crate) enum ElementMode {
    Passive,
    Active { table: TableIdx, offset: ConstExpr },
    Declarative,
}

pub(crate) enum DataMode {
    Passive,
    Active { memory: MemIdx, offset: ConstExpr },
}
