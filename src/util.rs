use wasm_bindgen::prelude::*;

#[macro_export]
macro_rules! log_rule {
    () => {
        cfg_if::cfg_if! {
            if #[cfg(test)] {
                pub fn log(s: &str) {
                    println!("{:?}", s);
                }
            } else {
                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(js_namespace = console)]
                    fn log(s: &str);
                }
            }
        }
    };
}
