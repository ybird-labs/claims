# Vendored: oxigraph JS/WebAssembly build

- Package: `oxigraph` (npm), version **0.5.9** — the official
  JavaScript/WebAssembly build of the same Oxigraph engine the spike uses
  as a Rust crate (also 0.5.9).
- Source: https://registry.npmjs.org/oxigraph/-/oxigraph-0.5.9.tgz
  (tarball sha256 `f1f449efd747a7355840bc78bddfda10d1a859d22949153eee6a37c814bbbb99`),
  obtained via `npm pack oxigraph@0.5.9`; the two files below are copied
  unmodified from the tarball's `package/` directory.
- License: **MIT OR Apache-2.0** (per the package metadata;
  https://github.com/oxigraph/oxigraph).
- Files (sha256):
  - `web.js` — wasm-bindgen "web" target ES module
    `a616e391868c6f3884c5e46b0f85253d6e860c49031448566053afccdb90b291`
  - `web_bg.wasm` — the engine compiled to WebAssembly
    `7b5c8669cfdd23cfd705e7a10c767dc3a1e0bacca18980da4c7bedfb33f9abcb`

Used by `src/export.rs` to build the self-contained
`out/carbon/sparql.html` workbench: `web.js` is inlined verbatim into a
`<script type="module">` (its `export` statements become inert local
bindings) and `web_bg.wasm` is embedded as base64 and instantiated from an
ArrayBuffer, so the page performs no network requests.
