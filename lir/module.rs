use crate::descriptors::LirCode;
use crate::descriptors::LirData;
use crate::descriptors::LirElement;
use crate::descriptors::LirExport;
use crate::descriptors::LirGlobal;
use crate::descriptors::LirImport;
use crate::descriptors::LirMemory;
use crate::descriptors::LirTable;
use crate::indices::FuncIdx;
use crate::indices::TypeIdx;
use crate::types::FuncType;

pub struct LirModule {
    type_section: Vec<FuncType>,
    // We ignore this for now :)
    import_section: Vec<LirImport>,
    fn_section: Vec<TypeIdx>,
    table_section: Vec<LirTable>,
    memory_section: Option<LirMemory>,
    global_section: Vec<LirGlobal>,
    export_section: Vec<LirExport>,
    start_section: Option<FuncIdx>,
    element_section: Vec<LirElement>,
    code_section: Vec<LirCode>,
    data_section: Vec<LirData>,
}

impl LirModule {
    pub fn new() -> Self {
        Self {
            type_section: Vec::with_capacity(12),
            import_section: Vec::with_capacity(4),
            fn_section: Vec::with_capacity(20),
            table_section: Vec::with_capacity(4),
            memory_section: None,
            global_section: Vec::with_capacity(4),
            export_section: Vec::with_capacity(8),
            start_section: None,
            element_section: Vec::with_capacity(4),
            code_section: Vec::with_capacity(20),
            data_section: Vec::with_capacity(4),
        }
    }

    pub fn add_type(&mut self, fn_type: FuncType) {
        self.type_section.push(fn_type);
    }

    pub fn add_func(&mut self, type_idx: u32) {
        self.fn_section.push(type_idx);
    }
}
