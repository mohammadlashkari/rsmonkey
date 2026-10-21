const source = document.getElementById("source");
const tokens = document.getElementById("tokens");
const ast = document.getElementById("ast");

// Runs one wasm function on the source and shows the result.
// A thrown error (e.g. from a Rust fn returning Result::Err) is shown in red.
function show(pre, run) {
  try {
    pre.textContent = run(source.value);
    pre.classList.remove("error");
  } catch (err) {
    pre.textContent = String(err);
    pre.classList.add("error");
  }
}

try {
  // Dynamic import so a missing build shows a message instead of a blank page
  const wasm = await import("./pkg/rsmonkey_web.js");
  await wasm.default();

  const update = () => {
    show(tokens, wasm.tokenize);
    show(ast, wasm.parse);
  };
  source.addEventListener("input", update);
  update();
} catch (err) {
  const message =
    "Could not load the wasm module.\n\n" +
    "Build it:  wasm-pack build crates/web --target web --out-dir www/pkg\n" +
    "Serve it:  python -m http.server -d crates/web/www 8000\n\n" +
    String(err);
  for (const pre of [tokens, ast]) {
    pre.textContent = message;
    pre.classList.add("error");
  }
}
