sdk/wasm/src/lib.rs

```rust
use wasm_bindgen::prelude::*;
use sdk_rust::{DeviceTrustClient, DeviceSignals};
use serde_wasm_bindgen::to_value;

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn collect_signals() -> JsValue {
    let signals = DeviceTrustClient::collect_signals().expect("signal collection failed");
    to_value(&signals).unwrap()
}

#[wasm_bindgen]
pub fn evaluate_trust(signals_js: JsValue) -> JsValue {
    let signals: DeviceSignals = serde_wasm_bindgen::from_value(signals_js).unwrap();
    let score = DeviceTrustClient::evaluate_trust(&signals);
    to_value(&score).unwrap()
}

#[wasm_bindgen]
pub fn generate_fingerprint(signals_js: JsValue) -> JsValue {
    let signals: DeviceSignals = serde_wasm_bindgen::from_value(signals_js).unwrap();
    let fp = DeviceTrustClient::generate_fingerprint(&signals);
    to_value(&fp).unwrap()
}
```
