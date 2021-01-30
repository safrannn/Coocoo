mod ast;
mod compiler;
mod image_functions;
mod image_library;
mod symbol;
mod util;
use wasm_bindgen::prelude::*;

log_rule!();

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub coocoo);
