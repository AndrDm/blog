# iching_wasm

`Rust + WASM` версия генератора I Ching с терминальным интерфейсом в браузере.

Проект устроен максимально просто:

- логика генерации написана на `Rust`;
- в браузере она работает через `WebAssembly`;
- интерфейс собран из обычных `HTML + CSS + JS`;
- публикация идёт как полностью статичная страница.

Демо на GitLab Pages: <https://i-ching-wasm-c50914.gitlab.io/>

## Структура

- `src/lib.rs` — логика генерации и экспорт функции в `WASM`.
- `public/index.html`, `public/styles.css`, `public/app.js` — статичный интерфейс.
- `public/pkg/` — артефакты, которые создаёт `wasm-pack`.
- `ARTICLE.md` — черновик статьи о переносе консольного приложения в `WASM`.

## Запуск

Пересобрать WASM и открыть `public/index.html` прямо в браузере: модуль `WASM` уже встраивается в `base64` и не требует отдельного запроса к `*.wasm`.

## Пересборка WASM

```bash
# выполнять из корня репозитория
wasm-pack build --target no-modules --release --out-dir public/pkg
echo "window.ICHING_WASM_BASE64 = \"$(base64 < public/pkg/iching_wasm_bg.wasm | tr -d '\n')\";" > public/pkg/iching_wasm_bg_base64.js
```

Что здесь происходит:

- `wasm-pack` собирает `Rust`-библиотеку в `WASM` и JS-обвязку;
- затем `iching_wasm_bg.wasm` превращается в `base64`;
- итоговый файл `public/pkg/iching_wasm_bg_base64.js` позволяет запускать страницу даже без отдельной загрузки `*.wasm`.

## Авторы

[AkaTorich](https://github.com/AkaTorich) — консольное приложение на `.NET`

`@EvgeneKopylov` — переписывание консольного приложения на `Rust`, сборка в `WASM` и публикация статической версии.
