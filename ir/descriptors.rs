use crate::indices::FuncIdx;
use crate::indices::GlobalIdx;
use crate::indices::MemIdx;
use crate::indices::TableIdx;
use crate::indices::TypeIdx;
use crate::types::GlobalType;
use crate::types::MemType;
use crate::types::TableType;

// TODO: Add instruction type
type Instruction = ();
type ConstInstruction = ();

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
