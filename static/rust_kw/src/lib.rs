use wasm_bindgen::prelude::*;
use chrono::prelude::*;

#[wasm_bindgen]
pub fn current_timestamp_with_kw() -> String {
    // Local time works in WASM when chrono has feature "wasmbind"
    let now = Local::now();

    let iso_week = now.iso_week().week();

    // Format: 2026-05-15 18:02:33 (KW20)
    format!(
        "{} (KW{})",
        now.format("%Y-%m-%d %H:%M:%S"),
        iso_week
    )
}
