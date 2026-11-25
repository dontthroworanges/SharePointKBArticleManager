fn main() {
    windows_reactor_setup::as_self_contained();

    // The app's icon, for the exe in Explorer, shortcuts and the taskbar. No rerun-if-changed:
    // that would stop the default rerun on any change, which the runtime copy above relies on.
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut resource = winresource::WindowsResource::new();
        resource.set_icon("assets/app.ico");
        // Task Manager and Explorer show these; they would otherwise be the crate's name.
        resource.set("FileDescription", "SharePoint Article Manager");
        resource.set("ProductName", "SharePoint Article Manager");
        resource
            .compile()
            .expect("failed to embed the app icon (needs the Windows SDK's rc.exe)");
    }
}
