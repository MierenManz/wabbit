pub struct LirModule {
    type_section: Vec<FnType>,
    // We ignore this for now :)
    // import_section: Vec<ImportDescriptor>,
    fn_section: Vec<u32>,
    table_section: Vec<TableDescriptor>,
    memory_section: Option<Limits>,
    global_section: Vec<GlobalDescriptor>,
    export_section: Vec<ExportDescriptor>,
    start_section: Option<u32>,
    element_section: Vec<TableInitializer>,
    code_section: Vec<CodeBlock>,
    data_section: Vec<DataInitializer>,
}

impl LirModule {
    pub fn new() -> Self {
        Self {
            type_section: Vec::with_capacity(12),
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

    pub (crate) fn add_type(&mut self, fn_type: FnType) {
        self.type_section.push(fn_type);
    }

    pub (crate) fn add_func(&mut self, type_idx: u32) {
        self.fn_section.push(type_idx);
    }
}