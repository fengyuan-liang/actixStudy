mod utils;

use wasm_bindgen::prelude::*;

// 调用前端的函数
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// 前端项目可以调用的rust函数
#[wasm_bindgen]
pub fn greet(s: &str) {
    alert(format!("Hello, {}", s).as_str());
}
