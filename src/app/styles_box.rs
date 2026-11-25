//! The Styles box shows each style in its own look, as the SharePoint web editor's does:
//! Heading 1 large and bold down to Paragraph in body text.
//!
//! Reactor's ComboBox only takes strings, so the native box is reached and each item's
//! container is restyled when the list opens. WinUI creates the containers after the
//! DropDownOpened event, so the styling waits for the UI thread to finish that layout.

use crate::winui as native;
use windows_core::{IUnknown, Interface, Result};

/// Font size (pixels) and weight of each item, in `STYLES` order: Heading 1–6, Paragraph.
const LOOKS: [(f64, u16); 7] = [
    (28.0, 600),
    (24.0, 600),
    (20.0, 600),
    (18.0, 600),
    (16.0, 600),
    (14.0, 700),
    (14.0, 400),
];

/// How many times to wait for the containers before giving up.
const ATTEMPTS: u32 = 5;

/// Restyles the box's items each time its list opens.
pub fn hook(object: &IUnknown) -> Result<()> {
    let combo: native::IComboBox = object.cast()?;
    let items: native::IItemContainerMapping = object.cast()?;
    let revoker = combo.DropDownOpened(move |_, _| style_items_later(items.clone(), ATTEMPTS))?;
    // The handler lives as long as the box does.
    std::mem::forget(revoker);
    Ok(())
}

/// Styles the items once the UI thread is idle, trying again if they are not there yet.
fn style_items_later(items: native::IItemContainerMapping, attempts: u32) {
    let Ok(queue) = native::DispatcherQueue::GetForCurrentThread()
        .and_then(|queue| queue.cast::<native::IDispatcherQueue>())
    else {
        return;
    };
    let handler = native::DispatcherQueueHandler::new(move || {
        if !style_items(&items) && attempts > 1 {
            style_items_later(items.clone(), attempts - 1);
        }
    });
    _ = queue.TryEnqueueWithPriority(native::DispatcherQueuePriority::Low, &handler);
}

/// Styles every item; false if some had no container yet.
fn style_items(items: &native::IItemContainerMapping) -> bool {
    let mut all = true;
    for (index, (size, weight)) in LOOKS.into_iter().enumerate() {
        match items
            .ContainerFromIndex(index as i32)
            .and_then(|container| container.cast::<native::IControl>())
        {
            Ok(control) => {
                _ = control.SetFontSize(size);
                _ = control.SetFontWeight(native::FontWeight { weight });
            }
            Err(_) => all = false,
        }
    }
    all
}
