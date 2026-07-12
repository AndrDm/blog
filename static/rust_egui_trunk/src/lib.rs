mod wasm_runner;

use egui::*;
use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

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

impl MyApp {
    fn ui(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Sieve (Rust + egui + WASM)");

            ui.add(
                Slider::new(&mut self.input_n, 10..=200_000)
                    .text("Max N"),
            );

            if ui.button("Compute").clicked() && !self.computing {
                self.computing = true;

                let n = self.input_n;
                let pending = self.pending_result.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    gloo_timers::future::TimeoutFuture::new(10).await;

                    let perf = web_sys::window().unwrap().performance().unwrap();
                    let start = perf.now();
                    let primes = sieve_of_eratosthenes(n);
                    let end = perf.now();

                    *pending.borrow_mut() = Some((primes, end - start));
                });
            }

            if let Some((primes, time)) = self.pending_result.borrow_mut().take() {
                self.primes = primes;
                self.last_runtime_ms = time;
                self.computing = false;
            }

            ui.separator();

            if self.computing {
                ui.label("Computing...");
            }

            ui.label(format!("Primes: {}", self.primes.len()));
            ui.label(format!("Time: {:.3} ms", self.last_runtime_ms));

            ui.separator();

            ScrollArea::vertical()
                .max_height(200.0)
                .show(ui, |ui| {
                    for p in &self.primes {
                        ui.label(p.to_string());
                    }
                });
        });
    }
}

// ---- WASM entry ----

#[wasm_bindgen]
pub async fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    wasm_runner::start(MyApp::default());

    Ok(())
}
