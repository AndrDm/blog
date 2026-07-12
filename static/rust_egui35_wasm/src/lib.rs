use eframe::egui;
use wasm_bindgen::prelude::*;

// ---- sieve ----
pub fn sieve_of_eratosthenes(n: usize) -> Vec<i32> {
    let mut primes = Vec::new();

    if n < 2 {
        return primes;
    }

    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=n {
        if is_prime[i] {
            primes.push(i as i32);
            let mut j = 2 * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
    }

    primes
}

// ---- app ----

#[derive(Default)]
struct MyApp {
    name: String,
    age: u32,
    counter: i32,
}

impl eframe::App for MyApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // Light blue background
        [
            200.0 / 255.0, // R
            220.0 / 255.0, // G
            255.0 / 255.0, // B
            1.0,           // A
        ]
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("Welcome to egui!");
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Your name: ");
            ui.text_edit_singleline(&mut self.name);
        });
        ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
        if ui.button("Increment Counter").clicked() {
            self.counter += 1;
        }
        ui.label(format!("Counter: {}", self.counter));
        ui.separator(); // ---------------------------------------------------
        if !self.name.is_empty() {
            ui.label(format!(
                "Hello, {}! You are {} years old.",
                self.name, self.age
            ));
        }
    }
}

// ---- WASM entry ----

//use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
pub async fn start(canvas_id: &str) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let runner = eframe::WebRunner::new();
    let options = eframe::WebOptions::default();

    // Get canvas element from DOM
    let canvas: HtmlCanvasElement = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(canvas_id)
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    runner
        .start(
            canvas,
            options,
            Box::new(|_cc| Ok(Box::new(MyApp::default()))),
        )
        .await
}
