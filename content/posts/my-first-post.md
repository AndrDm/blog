+++
date = '2026-05-12T04:35:34+02:00'
draft = false
title = 'My First Post'
+++

Test 1
{{< wasm-run module="rust_app/pkg/rust_app.js" function="sieve_of_eratosthenes" >}}

Test 2a
{{< wasm-run2 module="rust_hello/pkg/hello_wasm.js" function="generate_text" >}}

Test 2a
{{< wasm-run2 module="rust_weather/pkg/hello_weather.js" function="generate_report" >}}

Test 2b
{{< wasm-run2 module="iching_wasm/pkg/iching_wasm.js" function="generate_reading_text" >}}

Test 3 (egui)
{{< wasm-egui-lazy-start module="rust_egui/pkg/rust_hello.js" function="start" >}}

Test 4 (leptos)
{{< wasm-leptos module="rust_leptos\pkg\rust_leptos.js" >}}
