use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_text() -> String {

    let mut out = String::new();

    out.push_str("Hello, WASM!\n");
    out
}


#[cfg(test)]
mod tests {
    use super::generate_text;

    #[test]
    fn generated_text_contains_sections() {
        let text = generate_reading_text();
        assert!(text.contains("Hello, WASM"));
    }
}
