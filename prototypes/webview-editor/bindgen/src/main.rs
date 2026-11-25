//! Regenerates `../src/webview/bindings.rs` from the WebView2 SDK metadata.
//!
//! Usage: `cargo run -p prototype-bindgen -- <dir containing Microsoft.Web.WebView2.Core.winmd>`
//! The .winmd comes from the `Microsoft.Web.WebView2` NuGet package (its `lib` folder).

fn main() {
    let mut inputs: Vec<String> = std::env::args().skip(1).collect();
    assert!(!inputs.is_empty(), "pass the metadata directory as an argument");
    let out = concat!(env!("CARGO_MANIFEST_DIR"), "/../src/webview/bindings.rs");
    let filter = concat!(env!("CARGO_MANIFEST_DIR"), "/filter.txt");
    let mut args = vec!["--in".to_string(), "default".to_string()];
    args.append(&mut inputs);
    args.extend(
        ["--out", out, "--flat", "--minimal", "--filter-file", filter]
            .into_iter()
            .map(String::from),
    );
    windows_bindgen::bindgen(args);
}
