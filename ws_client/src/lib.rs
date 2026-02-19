// Import core wasm-bindgen functionality
// This allows Rust to interact with JavaScript.
use wasm_bindgen::prelude::*;

// JsCast is required for converting generic JsValue into specific JS types.
use wasm_bindgen::JsCast;

// web_sys provides bindings to browser APIs (WebSocket, DOM, Events, etc.)
use web_sys::{Event, MessageEvent, WebSocket};


// Expose this struct to JavaScript.
// Without #[wasm_bindgen], JS cannot see or use this struct.
#[wasm_bindgen]
pub struct WsClient {
    // This holds the browser's native WebSocket object.
    ws: WebSocket,
}


// Expose methods to JavaScript.
#[wasm_bindgen]
impl WsClient {

    // This constructor will be callable from JavaScript:
    // new WsClient(room_id, username)
    #[wasm_bindgen(constructor)]
    pub fn new(room_id: String, username: String) -> Result<WsClient, JsValue> {

        // Build WebSocket URL dynamically using room_id
        // Example: ws://127.0.0.1:3000/ws/room1
        let url = format!("ws://127.0.0.1:3000/ws/{}", room_id);

        // Create a browser WebSocket connection
        // This is NOT Rust TCP — this calls the browser's WebSocket API.
        let ws = WebSocket::new(&url)?;

        // Clone the WebSocket because closures require ownership.
        // The closure below will move this clone.
        let ws_clone = ws.clone();


        // Create a Rust closure that can be used as a JS callback.
        // FnMut because it can be called multiple times.
        let onopen = Closure::<dyn FnMut(_)>::new(move |_e: Event| {

            // When WebSocket connection opens,
            // send username to server (first message protocol).
            ws_clone.send_with_str(&username).unwrap();
        });

        // Register this closure as the WebSocket onopen callback.
        ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));

        // Prevent Rust from dropping the closure.
        // If we don't call forget(), the closure is freed,
        // and browser calling it later will crash.
        onopen.forget();



        let onmessage = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {

            // WebSocket message event contains JsValue.
            // We try to convert it into JsString.
            if let Ok(js_text) = e.data().dyn_into::<js_sys::JsString>() {

                // Access browser window
                let document = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap();

                // Get chat container element by ID
                let messages = document
                    .get_element_by_id("messages")
                    .unwrap();

                // Create a new <div> element
                let div = document.create_element("div").unwrap();

                // Add CSS class for styling
                div.set_class_name("message");

                // Convert JsString → Rust String
                let text = js_text.as_string().unwrap_or_default();

                // Insert message text into div
                // (Better: use set_text_content to avoid XSS)
                div.set_inner_html(&text);

                // Append new message div to chat container
                messages.append_child(&div).unwrap();
            }
        });

        // Register onmessage callback
        ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));

        // Prevent closure from being dropped
        onmessage.forget();


        // Return the WsClient instance to JavaScript
        Ok(WsClient { ws })
    }


    // This method is callable from JavaScript:
    // client.send("hello")
    pub fn send(&self, message: String) {

        // Send text message to server
        // If connection is closed, this will panic (unwrap).
        self.ws.send_with_str(&message).unwrap();
    }
}
