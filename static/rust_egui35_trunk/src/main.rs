#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

// desktop version
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([480.0, 320.0]),
        ..Default::default()
    };

    eframe::run_native(
        "egui desk",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_theme(egui::Theme::Light);
            Ok(Box::new(MyApp::default()))
        }),
    )
}

// web version
#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| {
                    cc.egui_ctx.set_theme(egui::Theme::Light);
                    Ok(Box::new(MyApp::default()))
                }),
            )
            .await;

        let loading_text = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("loading_text"));

        if let Some(loading_text) = loading_text {
            match start_result {
                Ok(_) => loading_text.remove(),
                Err(e) => {
                    loading_text.set_inner_html("<p> The app crashed (see console). </p>");
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}

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
