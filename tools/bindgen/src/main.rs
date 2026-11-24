//! Regenerates `src/winui/bindings.rs` from the Windows App SDK metadata.
//!
//! Usage: `cargo run -p bindgen -- <dir containing Microsoft.UI.Text.winmd and Microsoft.UI.Xaml.winmd> [Microsoft.UI.winmd dir]`
//! The .winmd files come from the `Microsoft.WindowsAppSDK.WinUI` and
//! `Microsoft.WindowsAppSDK.InteractiveExperiences` NuGet packages.

fn main() {
    let mut inputs: Vec<String> = std::env::args().skip(1).collect();
    assert!(
        !inputs.is_empty(),
        "pass the metadata directories as arguments"
    );
    let out = concat!(env!("CARGO_MANIFEST_DIR"), "/../../src/winui/bindings.rs");
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
