use super::ast::*;
use super::coocoo::ProgramParser;
use super::image_library::*;
use super::log_rule;
use super::symbol::*;
use std::collections::HashMap;
use walrus::FunctionId;
use walrus::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

log_rule!();

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

    fn import_lib(&mut self, symbol_table: &mut SymbolTable) {
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

            symbol_table.insert(
                name.clone(),
                Attribute::Func(func_id, args.to_vec(), result.to_vec()),
            );
        }
    }

    fn import_images(
        &mut self,
        builder: &mut InstrSeqBuilder,
        image_names: &Vec<String>,
        symbol_table: &mut SymbolTable,
    ) {
        for i in 0..image_names.len() as i32 {
            let image_name = image_names[i as usize].clone().trim().to_string();
            builder.i32_const(i);
            let image_id = self.module.locals.add(walrus::ValType::I32);
            builder.local_set(image_id);
            let image = Image::new(i);
            symbol_table.insert(image_name.clone(), Attribute::Image(image_id, Some(image)));
        }
    }

    fn compile(&mut self, src: String, image_names: Vec<String>) -> Vec<u8> {
        let mut symbol_table: SymbolTable = SymbolTable::new();
        self.import_lib(&mut symbol_table);

        self.src = "func main(){".to_string() + &src + &"}".to_string();
        let functions = ProgramParser::new().parse(&self.src).unwrap();
        let function = &functions[0];
        let mut function_builder = FunctionBuilder::new(&mut self.module.types, &vec![], &[]);
        let mut builder: InstrSeqBuilder = function_builder.func_body();
        self.import_images(&mut builder, &image_names, &mut symbol_table);
        function.compile(&mut self.module, &mut builder, &mut symbol_table);

        let function_id = function_builder.finish(vec![], &mut self.module.funcs);

        self.module
            .exports
            .add(&function.prototype.identifier, function_id);

        // IMAGE_LIBRARY
        //     .lock()
        //     .unwrap()
        //     .add_export_names(item_tracker.get_image_names());

        // log(&format!("program: {:?}", function));
        self.module.emit_wasm()
    }

    pub fn run(&mut self, src: String, image_names: Vec<String>) -> Vec<u8> {
        self.compile(src, image_names)
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
        (vec![ValType::I32, ValType::I32], vec![ValType::I32]),
    );
    return lib_func_list;
}

#[wasm_bindgen]
pub fn code_to_wasm(src: String, image_names: &JsValue) -> Vec<u8> {
    let mut compiler = Compiler::new();
    let image_names: Vec<String> = parse_image_names(image_names);
    compiler.run(src, image_names)
}

#[cfg(test)]
#[cfg(any(target_os = "linux", target_os = "macos"))]
mod tests {
    use super::*;
    use wabt::*;
    use wasmer::*;
    use wasmer_compiler_cranelift::Cranelift;
    use wasmer_engine_jit::JIT;
    use wasmer_runtime::*;

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
            "func main(){number1 = 1;
                img1 = blank_image(10,10);}",
        ) {
            Ok(result) => result,
            Err(e) => panic!(e),
        };
    }

    #[test]
    fn rust_test3() {
        let wasm_bytes: Vec<u8> = vec![
            0, 97, 115, 109, 1, 0, 0, 0, 1, 132, 128, 128, 128, 0, 1, 96, 0, 0, 3, 130, 128, 128,
            128, 0, 1, 0, 7, 136, 128, 128, 128, 0, 1, 4, 109, 97, 105, 110, 0, 0, 10, 132, 128,
            128, 128, 0, 1, 2, 0, 11, 0, 167, 128, 128, 128, 0, 9, 112, 114, 111, 100, 117, 99,
            101, 114, 115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 1, 6,
            119, 97, 108, 114, 117, 115, 6, 48, 46, 49, 56, 46, 48,
        ];
        // module
        //     (type (;0;) (func))
        //     (func (;0;) (type 0))
        //     (export "main" (func 0)))

        let store = Store::new(&JIT::new(&Cranelift::default()).engine());
        let module = wasmer::Module::new(&store, &wasm_bytes[..]).expect("create module");
        let import_object = wasmer::imports! {};
        let instance = wasmer::Instance::new(&module, &import_object).expect("instantiate module");
        let _func_main: NativeFunc = instance
            .exports
            .get_native_function("main")
            .expect("add_one function in Wasm module");
        let result = _func_main.call().unwrap();
        log(&format!("Result: {:?}", result));
    }

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
