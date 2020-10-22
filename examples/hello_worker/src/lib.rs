use wasm_bindgen::prelude::*;
use webworker::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub async fn run() {
    let mut worker = Worker::<String>::new("worker.js");

    if let Some(msg) = worker.messages().next().await {
        log(&msg);
    };
}
