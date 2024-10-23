use super::types::LirCode;
use super::types::LirData;
use super::types::LirElement;
use super::types::LirExport;
use super::types::LirGlobal;
use super::types::LirImport;
use super::types::LirMemory;
use super::types::LirTable;

use crate::common::FuncIdx;
use crate::common::FuncType;
use crate::common::TypeIdx;

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
    #[doc(hidden)]
    /// I will find you if you use this outside of the waffle crates.
    pub fn __internal_new_do_not_use_or_you_will_be_chased_and_punished(
        type_section: Vec<FuncType>,
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
    ) -> Self {
        Self {
            type_section,
            import_section,
            fn_section,
            table_section,
            memory_section,
            global_section,
            export_section,
            start_section,
            element_section,
            code_section,
            data_section,
        }
    }

    pub fn new() -> Self {
        Self {
            type_section: vec![],
            import_section: vec![],
            fn_section: vec![],
            table_section: vec![],
            memory_section: None,
            global_section: vec![],
            export_section: vec![],
            start_section: None,
            element_section: vec![],
            code_section: vec![],
            data_section: vec![],
        }
    }

    pub fn add_type(&mut self, fn_type: FuncType) {
        self.type_section.push(fn_type);
    }

    pub fn add_func(&mut self, type_idx: TypeIdx) {
        self.fn_section.push(type_idx);
    }
}
