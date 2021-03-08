use super::ast::*;
use super::coocoo::ProgramParser;
// use super::image_library::*;
use super::log_rule;
use super::symbol::*;
use std::collections::HashMap;
// use walrus::FunctionId;
use id_arena::*;
use walrus::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

log_rule!();

#[derive(Clone)]
pub enum MemoryValue {
    walrus_id(Id<walrus::Local>),
    i32(i32),
}

pub struct Memory {
    pub id: walrus::MemoryId,
    last_offset: u32,
}

impl Memory {
    pub fn new(module: &mut walrus::Module) -> Self {
        return Memory {
            id: module.memories.add_local(false, 10, Some(100)),
            last_offset: 0,
        };
    }

    pub fn store(
        &mut self,
        builder: &mut InstrSeqBuilder,
        offset: Option<u32>,
        value: Vec<MemoryValue>,
    ) -> (walrus::MemoryId, u32) {
        let mut offset = if offset.is_some() {
            offset.unwrap()
        } else {
            self.last_offset.clone()
        };
        let start_offset = offset;
        let align = 2;
        let n = value.len();
        for i in 0..n {
            builder.i32_const(offset as i32);
            offset += u32::pow(2, align);

            match value[i] {
                MemoryValue::walrus_id(id) => {
                    builder.local_get(id);
                }
                MemoryValue::i32(number) => {
                    builder.i32_const(number);
                }
            }

            builder.store(
                self.id,
                walrus::ir::StoreKind::I32 { atomic: false },
                walrus::ir::MemArg { align, offset: 0 },
            );
        }
        self.last_offset = self.last_offset.max(offset);
        return (self.id.clone(), start_offset);
    }

    pub fn copy(
        &mut self,
        builder: &mut InstrSeqBuilder,
        offset_1: u32,
        offset_2: u32,
        length: u32,
    ) {
        let align = 2;
        let mut offset_1 = offset_1;
        let mut offset_2 = offset_2;
        for _ in 0..length as usize {
            builder.i32_const(offset_1 as i32);
            builder.i32_const(offset_2 as i32);
            builder.load(
                self.id,
                walrus::ir::LoadKind::I32 { atomic: false },
                walrus::ir::MemArg { align, offset: 0 },
            );

            builder.store(
                self.id,
                walrus::ir::StoreKind::I32 { atomic: false },
                walrus::ir::MemArg { align, offset: 0 },
            );
            offset_1 += u32::pow(2, align);
            offset_2 += u32::pow(2, align);
        }
    }
}

pub struct Compiler {
    module: walrus::Module,
    src: String,
    symbol_table: SymbolTable,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            module: walrus::Module::with_config(ModuleConfig::new()),
            src: "".to_string(),
            symbol_table: SymbolTable::new(),
        }
    }

    fn import_lib(&mut self) {
        let lib_func_list = library_function_list();
        for (name, (args, result)) in lib_func_list.iter() {
            let type_id = if let Some(t_id) = self.module.types.find(args, result) {
                t_id
            } else {
                self.module.types.add(args, result)
            };
            let (func_id, import_id) = self.module.add_import_func("env", name, type_id);
            self.module
                .globals
                .add_import(walrus::ValType::Funcref, false, import_id);

            self.symbol_table.insert(
                name.clone(),
                Attribute::Func(func_id, args.to_vec(), result.to_vec()),
            );
        }
    }

    fn import_images(&mut self, builder: &mut InstrSeqBuilder, image_names: &Vec<String>) {
        for i in 0..image_names.len() as i32 {
            let image_name = image_names[i as usize].clone().trim().to_string();
            builder.i32_const(i);
            let image_id = self.module.locals.add(walrus::ValType::I32);
            builder.local_set(image_id);
            let image = Image::new(i);
            self.symbol_table
                .insert(image_name.clone(), Attribute::Image(image_id, Some(image)));
        }
    }

    fn compile(&mut self, src: String, image_names: Vec<String>) -> Vec<u8> {
        let mut symbol_table: SymbolTable = SymbolTable::new();
        self.import_lib();

        self.src = "func main(){".to_string() + &src + &"}".to_string();
        let functions = ProgramParser::new().parse(&self.src).unwrap();
        let function = &functions[0];
        let mut function_builder = FunctionBuilder::new(&mut self.module.types, &vec![], &[]);
        let mut builder: InstrSeqBuilder = function_builder.func_body();
        self.import_images(&mut builder, &image_names);

        let mut memory = Memory::new(&mut self.module);
        let function_compile_result = function.compile(
            &mut self.module,
            &mut builder,
            &mut self.symbol_table,
            &mut memory,
        );
        if function_compile_result.is_ok() {
            let function_id = function_builder.finish(vec![], &mut self.module.funcs);

            self.module
                .exports
                .add(&function.prototype.identifier, function_id);
            self.module.exports.add("mem", memory.id);

            self.module.emit_wasm()
        } else {
            vec![]
        }
    }

    pub fn export(&mut self, src: String, image_names: Vec<String>) -> Vec<JsValue> {
        vec![
            JsValue::from_serde(&self.compile(src, image_names)).unwrap(),
            self.symbol_table.library_tracker.export_images(),
            self.symbol_table.library_tracker.export_materials(),
        ]
    }
}

fn parse_image_names(image_file_names: &JsValue) -> Vec<String> {
    match image_file_names.into_serde::<Vec<String>>() {
        Ok(names) => names,
        Err(e) => {
            log(&format!(
                "Err: {:?}, failed to pass image file names from js to wasm",
                e
            ));
            vec![]
        }
    }
}

fn library_function_list() -> HashMap<String, (Vec<walrus::ValType>, Vec<walrus::ValType>)> {
    let mut lib_func_list: HashMap<String, (Vec<walrus::ValType>, Vec<walrus::ValType>)> =
        HashMap::new();
    lib_func_list.insert("logger".to_string(), (vec![ValType::I32], vec![]));
    lib_func_list.insert(
        "darken".to_string(),
        (vec![ValType::I32, ValType::I32], vec![ValType::I32]),
    );
    lib_func_list.insert(
        "blank_image".to_string(),
        (vec![ValType::I32, ValType::I32], vec![ValType::I32]),
    );
    lib_func_list.insert(
        "grayscale".to_string(),
        (vec![ValType::I32], vec![ValType::I32]),
    );
    return lib_func_list;
}

#[wasm_bindgen]
pub fn code_to_wasm(src: String, image_names: &JsValue) -> Vec<JsValue> {
    let mut compiler = Compiler::new();
    compiler.export(src, parse_image_names(image_names))
}

#[cfg(test)]
#[cfg(any(target_os = "linux", target_os = "macos"))]
mod tests {
    use super::*;
    use wabt::*;
    use wasmer::*;
    use wasmer_compiler_cranelift::Cranelift;
    use wasmer_engine_jit::JIT;
    // use wasmer_runtime::*;

    #[test]
    fn rust_test0() {
        let test_wasm: Vec<u8> = vec![
            0, 97, 115, 109, 1, 0, 0, 0, 1, 7, 1, 96, 2, 127, 127, 1, 127, 3, 2, 1, 0, 7, 10, 1, 6,
            97, 100, 100, 84, 119, 111, 0, 0, 10, 9, 1, 7, 0, 32, 0, 32, 1, 106, 11, 0, 14, 4, 110,
            97, 109, 101, 2, 7, 1, 0, 2, 0, 0, 1, 0,
        ];

        let mut module = match walrus::Module::from_buffer(&test_wasm) {
            Ok(module) => module,
            Err(_e) => {
                return;
            }
        };
        let wasm_emit = module.emit_wasm();

        assert_eq!(wasm2wat(&test_wasm), wasm2wat(&wasm_emit));
    }

    #[test]
    fn rust_test1() {
        let test_output = "(module
            (type $t0 (func))
            (func $main (type $t0)
              (local $l0 i32) 
              (local.set $l0
                (i32.const 20))
            (export \"main\" (func $main)))"
            .to_string();
        // let test_output_wasm = std::str::from_utf8(&test_output).unwrap().to_string();

        let mut compiler = super::Compiler::new();
        let wasm_emit = compiler.compile("number1 = 20;".to_string(), vec![]);
        dbg!(&test_output, wabt::wasm2wat(&wasm_emit));
    }

    #[test]
    fn rust_test2() {
        let _functions = match ProgramParser::new().parse(
            "func main(){
                var num1:N;
                var num2:N = 1;
                var img1:I = blank_image(10,10);
            }",
        ) {
            Ok(result) => result,
            Err(e) => panic!(e),
        };
    }

    // #[test]
    // fn rust_test3() {
    //     let wasm_bytes: Vec<u8> = vec![
    //         0, 97, 115, 109, 1, 0, 0, 0, 1, 132, 128, 128, 128, 0, 1, 96, 0, 0, 3, 130, 128, 128,
    //         128, 0, 1, 0, 7, 136, 128, 128, 128, 0, 1, 4, 109, 97, 105, 110, 0, 0, 10, 132, 128,
    //         128, 128, 0, 1, 2, 0, 11, 0, 167, 128, 128, 128, 0, 9, 112, 114, 111, 100, 117, 99,
    //         101, 114, 115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 1, 6,
    //         119, 97, 108, 114, 117, 115, 6, 48, 46, 49, 56, 46, 48,
    //     ];
    //     // module
    //     //     (type (;0;) (func))
    //     //     (func (;0;) (type 0))
    //     //     (export "main" (func 0)))

    //     let store = Store::new(&JIT::new(&Cranelift::default()).engine());
    //     let module = wasmer::Module::new(&store, &wasm_bytes[..]).expect("create module");
    //     let import_object = wasmer::imports! {};
    //     let instance = wasmer::Instance::new(&module, &import_object).expect("instantiate module");
    //     let _func_main: NativeFunc = instance
    //         .exports
    //         .get_native_function("main")
    //         .expect("add_one function in Wasm module");
    //     let result = _func_main.call().unwrap();
    //     log(&format!("Result: {:?}", result));
    // }

    // #[test]
    // fn rust_test4() {
    //     let src = "
    //         img2 = blank_image(10,20);
    //     ";
    //     let mut compiler = super::Compiler::new();
    //     let wasm_bytes = compiler.run(src.to_string(), vec![]);
    //     // (module
    //     //     (type (;0;) (func))
    //     //     (type (;1;) (func (param i32 i32) (result i32)))
    //     //     (import "./coocoo_bg.wasm" "darken" (func (;0;) (type 1)))
    //     //     (import "./coocoo_bg.wasm" "blank_image" (func (;1;) (type 1)))
    //     //     (func (;2;) (type 0))
    //     //     (export "main" (func 2)))

    //     let blank_image = move |width: i32, height: i32| -> i32 { width + height };
    //     let import_object = wasmer_runtime::imports! {
    //         "env" => {
    //             "blank_image" => func!(blank_image),
    //         },
    //     };
    //     let instance = match wasmer_runtime::instantiate(&wasm_bytes, &import_object) {
    //         Ok(result) => result,
    //         Err(e) => {
    //             dbg!(e);
    //             panic!();
    //         }
    //     };

    //     let main: Func = match instance.exports.get("main") {
    //         Ok(result) => result,
    //         Err(e) => panic!(e),
    //     };

    //     let _result = match main.call() {
    //         Ok(r) => r,
    //         Err(e) => panic!(e),
    //     };
    // }
}
