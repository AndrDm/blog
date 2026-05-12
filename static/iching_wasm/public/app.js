const output = document.getElementById("resultText");
const button = document.getElementById("generateBtn");
const statusLine = document.getElementById("statusLine");

function base64ToBytes(base64) {
  const binary = atob(base64);
  const bytes = new Uint8Array(binary.length);

  for (let i = 0; i < binary.length; i += 1) {
    bytes[i] = binary.charCodeAt(i);
  }

  return bytes;
}

function bootstrap() {
  button.disabled = true;
  statusLine.textContent = "Инициализация WASM...";

  try {
    if (typeof wasm_bindgen === "undefined") {
      throw new Error("wasm_bindgen runtime not found");
    }
    if (typeof window.ICHING_WASM_BASE64 !== "string") {
      throw new Error("embedded wasm bytes not found");
    }

    const wasmBytes = base64ToBytes(window.ICHING_WASM_BASE64);
    wasm_bindgen.initSync({ module: wasmBytes });

    statusLine.textContent = "WASM загружен (file:// совместимо). Нажмите ./generate_reading";
    button.disabled = false;
    runGeneration();
  } catch (error) {
    console.error(error);
    output.textContent = String(error);
    statusLine.textContent = "Не удалось инициализировать WASM.";
  }
}

function runGeneration() {
  try {
    const text = wasm_bindgen.generate_reading_text();
    output.textContent = text;
    statusLine.textContent = `Готово • ${new Date().toLocaleString("ru-RU")}`;
  } catch (error) {
    console.error(error);
    output.textContent = "Ошибка выполнения генерации в WASM.";
    statusLine.textContent = "Выполнение завершилось с ошибкой.";
  }
}

button.addEventListener("click", runGeneration);
bootstrap();
