fn main() {
    let css = r#"body {
        background-color: #fff;
        margin: 0;
        overflow: hidden;
    }"#;

    cargo_run_wasm::run_wasm_with_css(css);
}
