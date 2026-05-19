---
title: Weather
authorbox: false
sidebar: true
menu: main
date: 2024-11-07
---

Weather in Ahrensburg (this is Rust program, compiled to WASM):
{{< wasm-run4 module="rust_weather/pkg/hello_weather.js" function="fetch_weather_report" >}}

Weather in Ahrensburg

{{< iframe_ndr "https://www.ndr.de/nachrichten/wetter/wetterplzsuche100.html?plz=22926" >}}

Rain Radar

{{< iframe_ndr_radar "https://www.ndr.de/nachrichten/wetter/index.html?radaron" >}}

News

<iframe src="https://www.ndr.de/nachrichten/info/audio-407050~player.html" width="512" height="180" allowfullscreen frameBorder="0" scrolling="no"></iframe>
