// Example chat client for use in webassembly.
//
// I decided to put the websocket code on the JS side instead of using
// web_sys, and put message encoding on the Rust side so that I can
// share it with the server code. The JS side calls handle_input on
// the Rust side, and the Rust side calls send_to_server on the JS
// side. The encoding of messages to and from [u8] is on the Rust
// side.

// License: Apache-v2.0

mod common;

#[cfg(target_arch = "wasm32")]
mod client {
    use wasm_bindgen::prelude::*;
    use crate::common::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console, js_name = log)]
        fn log(s: &str);

        #[wasm_bindgen]
        fn send_to_server(msg: &[u8]);

        #[wasm_bindgen]
        fn add_to_output(from: &str, text: &str);

        #[wasm_bindgen]
        fn set_name(name: &str);
        
        #[wasm_bindgen]
        fn set_connection_count(count: u32);
    }

    #[wasm_bindgen]
    pub fn connected() {
    }

    #[wasm_bindgen]
    pub fn handle_input(text: &str) {
        let reply = ClientToServerMessage::Chat{text: String::from(text)};
        let encoded: Vec<u8> = bincode::serialize(&reply).unwrap();
        send_to_server(&encoded);
    }

    #[wasm_bindgen]
    pub fn handle_message(data: &[u8]) {
        let request: ServerToClientMessage = bincode::deserialize(&data).unwrap();
        match request {
            ServerToClientMessage::Chat{from, text} => add_to_output(&from, &text),
            ServerToClientMessage::SetName{name} => set_name(&name),
            ServerToClientMessage::SetConnectionCount{count} => set_connection_count(count),
        };
    }
}

#[allow(dead_code)]
fn main() {}
