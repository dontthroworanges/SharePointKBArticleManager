//! Window-wide keys while focus is in the app's own controls (the ribbon, the panes). Keys
//! typed in the page come from the page instead (see `web/editor.js`), as WebView2 keeps
//! them to itself.

use super::{Msg, Shortcut};
use crate::winui::*;
use windows_core::{Interface, Result};
use windows_reactor::LocalSender;

#[link(name = "user32")]
unsafe extern "system" {
    fn GetKeyState(key: i32) -> i16;
}

fn is_down(key: VirtualKey) -> bool {
    // SAFETY: GetKeyState has no preconditions.
    unsafe { GetKeyState(key.0) < 0 }
}

/// The + and - keys of the main keyboard, which VirtualKey does not name.
const VK_OEM_PLUS: i32 = 0xBB;
const VK_OEM_MINUS: i32 = 0xBD;

/// Shortcuts that work wherever focus is in the window.
pub fn app_shortcut(key: VirtualKey) -> Option<Shortcut> {
    let shift = is_down(VirtualKey::Shift);
    let control = is_down(VirtualKey::Control);
    if is_down(VirtualKey::Menu) {
        return None;
    }
    // Function keys, as in Word and Notepad.
    match key {
        VirtualKey::F3 if !control => {
            return Some(if shift {
                Shortcut::FindPrevious
            } else {
                Shortcut::FindNext
            });
        }
        VirtualKey::F12 if control && !shift => return Some(Shortcut::Open),
        VirtualKey::F12 if shift && !control => return Some(Shortcut::Save),
        VirtualKey::F12 if !control => return Some(Shortcut::SaveAs),
        _ => {}
    }
    if !control {
        return None;
    }
    Some(match key {
        VirtualKey::S if shift => Shortcut::SaveAs,
        VirtualKey::S => Shortcut::Save,
        VirtualKey::O if !shift => Shortcut::Open,
        VirtualKey::N if !shift => Shortcut::New,
        VirtualKey::F if !shift => Shortcut::Find,
        VirtualKey::H if !shift => Shortcut::Replace,
        // Ctrl with + or - on the main keyboard (also =, the unshifted +) or the number pad.
        key if key.0 == VK_OEM_PLUS || key == VirtualKey::Add => Shortcut::ZoomIn,
        key if key.0 == VK_OEM_MINUS || key == VirtualKey::Subtract => Shortcut::ZoomOut,
        VirtualKey::Number0 | VirtualKey::NumberPad0 if !shift => Shortcut::ZoomReset,
        _ => return None,
    })
}

/// The page's names for the same shortcuts (see `appShortcut` in `web/editor.js`).
pub fn page_shortcut(name: &str) -> Option<Shortcut> {
    Some(match name {
        "save" => Shortcut::Save,
        "saveAs" => Shortcut::SaveAs,
        "open" => Shortcut::Open,
        "new" => Shortcut::New,
        "find" => Shortcut::Find,
        "replace" => Shortcut::Replace,
        "findNext" => Shortcut::FindNext,
        "findPrevious" => Shortcut::FindPrevious,
        "zoomIn" => Shortcut::ZoomIn,
        "zoomOut" => Shortcut::ZoomOut,
        "zoomReset" => Shortcut::ZoomReset,
        "link" => Shortcut::Link,
        _ => return None,
    })
}

/// Hooks the window's root element so `app_shortcut`s are seen before any control handles them.
pub fn hook_root(object: windows_core::IUnknown, sender: LocalSender<Msg>) -> Result<()> {
    let element: IUIElement = object.cast()?;
    let revoker = element.PreviewKeyDown(move |_, args| {
        let Some(args) = args.as_ref() else { return };
        let Ok(args) = args.cast::<IKeyRoutedEventArgs>() else {
            return;
        };
        if let Some(shortcut) = args.Key().ok().and_then(app_shortcut) {
            _ = args.SetHandled(true);
            sender.send(Msg::AppShortcut(shortcut));
        }
    })?;
    // The root lives as long as the window, and so does its handler.
    std::mem::forget(revoker);
    Ok(())
}
