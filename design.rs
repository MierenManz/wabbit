fn main() {
    println!("Hello, world!");
}

struct Module {
    // Imports do not exist!
    exports: (),
    functions: (),

    global_initializers: (),
    memory_initializer: (),
    table_initializers: (),
}

impl Module {
    fn exports(&self) -> Vec<(String, ExportKind)> {}
}

struct DecodedModule {
    type_section: (),
    // ..
}

impl DecodedModule {
    fn new() -> Self {}
    fn add_type() -> Self {}

    fn instantiate() -> Module {}
}

struct Decoder {
    fd: (),
}

impl Decoder {
    fn new() -> Self {}

    fn decode_type_section(&mut self, module: &mut DecodedModule) -> Result<(), DecodingError> {
        //
        module.add_type();
    }
}
fn validate_module(module: &DecodedModule) -> Result<(), ValidationError> {
    // Big fn checking all sections
}

enum Permissions {
    IO = 1,
    FS = 1 << 1,
    Net = 1 << 2,
}

struct Engine {
    call_stack: Vec<FnState>,

    /// Runtime determined global values
    globals: Vec<Value>,
    /// Runtime determined memory page
    memory: Option<Memory>,
    /// Runtime determined (value) tables
    tables: Vec<Table>,
}

impl Engine {
    fn from_module(module: (), perms: ()) -> Self {
        Self {}
    }

    fn start(&mut self) -> Result<(), ()> {
        Ok(())
    }

    // returns None if memory does not exist
    // returns Err if memory is not exported
    fn borrow_memory(&self) -> Option<Result<&Memory, ()>> {
        Some(Ok(&self.memory))
    }

    // returns None if memory does not exist
    // returns Err if memory is not exported
    fn borrow_memory_mut(&mut self) -> Option<Result<&mut Memory, ()>> {
        Some(Ok(&mut self.memory))
    }

    // returns None if global does not exist
    // returns Err if global is not exported
    fn borrow_global(&self, name: &str) -> Option<Result<&Value, ()>> {}

    // returns None if global does not exist
    // returns Err if global is not exported
    fn borrow_global_mut(&mut self, name: &str) -> Option<Result<&mut Value, ()>> {}

    // returns None if table does not exist
    // returns Err if table is not exported
    fn borrow_table(&self, name: &str) -> Option<Result<&Table, ()>> {}

    // returns None if table does not exist
    // returns Err if table is not exported
    fn borrow_table_mut(&mut self, name: &str) -> Option<Result<&mut Table, ()>> {}

    // returns Err if fn does not exist or is not exported
    fn execute_fn(&mut self, fn_name: &str, args: Vec<Value>) -> Result<(), ()> {
        Ok(())
    }
}

struct FnState {
    stack: Vec<Value>,
    scope: Vec<u32>,
    current_locals: Vec<Value>,
    tail_call: bool,
}

fn x() {
    let mut engine = Engine::from_module((), ());

    engine.start();

    let mut mem = engine.borrow_memory_mut();

    mem[0] = 0u32;

    mem[0] = 12u8;

    engine.execute_fn("add_from_mem", []);

    let maybe_access = engine.borrow_global("blaaa").unwrap();
    let global = maybe_access.unwrap();
}
