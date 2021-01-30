mod ast;
mod compiler;
mod image_functions;
mod image_library;
mod symbol;
mod util;

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub coocoo);

#[macro_export]
macro_rules! log_rule {
    () => {
        cfg_if::cfg_if! {
            if #[cfg(test)] {
                pub fn log(s: &str) {
                    println!("{:?}", s);
                }
            } else {
                use wasm_bindgen::prelude::*;
                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(js_namespace = console)]
                    fn log(s: &str);
                }
            }
        }
    };
}
