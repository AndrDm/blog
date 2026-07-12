use eframe::egui;
use wasm_bindgen::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

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

pub struct MyApp {
    input_n: usize,
    primes: Vec<i32>,
    last_runtime_ms: f64,
    computing: bool,

    // shared async result (IMPORTANT FIX)
    pending_result: Rc<RefCell<Option<(Vec<i32>, f64)>>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input_n: 100,
            primes: Vec::new(),
            last_runtime_ms: 0.0,
            computing: false,
            pending_result: Rc::new(RefCell::new(None)),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //  force continuous repaint (WASM stability)
        ctx.request_repaint();
ctx.set_pixels_per_point(1.0);

        //  apply async result safely
        if let Some((primes, time)) = self.pending_result.borrow_mut().take() {
            self.primes = primes;
            self.last_runtime_ms = time;
            self.computing = false;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Sieve (Rust + egui + WASM)");

            ui.add(
                egui::Slider::new(&mut self.input_n, 10..=200_000)
                    .text("Max N"),
            );

if ui.button("Toggle Theme").clicked() {
    if ctx.style().visuals.dark_mode {
        ctx.set_visuals(egui::Visuals::light());
    } else {
        ctx.set_visuals(egui::Visuals::dark());
    }
}


            if ui.button("Compute").clicked() && !self.computing {
                self.computing = true;

                let n = self.input_n;
                let pending = self.pending_result.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    //  VERY IMPORTANT: let browser render first
                    gloo_timers::future::TimeoutFuture::new(10).await;

                    let performance = web_sys::window()
                        .unwrap()
                        .performance()
                        .unwrap();

                    let start = performance.now();

                    let primes = sieve_of_eratosthenes(n);

                    let end = performance.now();

                    //  store result safely
                    *pending.borrow_mut() = Some((primes, end - start));
                });
            }

            ui.separator();

            if self.computing {
                ui.label("Computing...");
            }

            ui.label(format!("Primes: {}", self.primes.len()));
            ui.label(format!("Time: {:.3} ms", self.last_runtime_ms));


ui.separator();

ui.label(format!("Primes: {}", self.primes.len()));
ui.label(format!("Time: {:.3} ms", self.last_runtime_ms));

ui.separator();

//  Scrollable list of primes
egui::ScrollArea::vertical()
    .max_height(200.0)
    .show(ui, |ui| {
        if self.primes.is_empty() {
            ui.label("No primes computed yet.");
        } else {
            //  limit rendering for performance
            let max_display = 1000;
            let display_count = self.primes.len().min(max_display);

            for p in &self.primes[..display_count] {
                ui.label(p.to_string());
            }

            if self.primes.len() > max_display {
                ui.label(format!(
                    "... and {} more",
                    self.primes.len() - max_display
                ));
            }
        }
    });


        });
    }
}

// ---- WASM entry ----

#[wasm_bindgen]
pub async fn start(canvas_id: &str) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let runner = eframe::WebRunner::new();

    let mut options = eframe::WebOptions::default();
    options.follow_system_theme = false;
//options.dpi_scale = 1.0; //  NEW (if available in your version)

    runner
        .start(
            canvas_id,
            options,
            Box::new(|_cc| Box::new(MyApp::default())),
        )
        .await
}
