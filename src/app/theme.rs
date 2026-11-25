//! Light and dark themes, and the accent colors the app picks itself.
//!
//! Most of the window follows the theme through WinUI theme brushes (`Brush::Theme`), which XAML
//! resolves for each element. A few colors have to be picked here instead: the title and tab
//! text and the window buttons (over Mica), and the ribbon's highlight colors, which are set as
//! theme resource overrides and so must be fixed colors.

use crate::winui as native;
use std::cell::Cell;
use windows_core::{Interface, Result};
use windows_reactor::Color;

/// The theme the user picked in View › Theme.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ThemeChoice {
    /// Follow the Windows app mode (Settings › Personalization › Colors).
    #[default]
    System,
    Light,
    Dark,
}

pub const THEMES: [(&str, ThemeChoice); 3] = [
    ("Use system setting", ThemeChoice::System),
    ("Light", ThemeChoice::Light),
    ("Dark", ThemeChoice::Dark),
];

/// Windows' default accent blue, used if the accent color cannot be read.
const DEFAULT_ACCENT: Color = Color::rgb(0x00, 0x78, 0xD4);
/// The Mica backdrop's base tone in each theme (before the wallpaper tints it), for the
/// window buttons' faded (inactive) color.
const LIGHT_MICA: Color = Color::rgb(0xF3, 0xF3, 0xF3);
const DARK_MICA: Color = Color::rgb(0x20, 0x20, 0x20);
/// WinUI's text colors and the tertiary background the ribbon uses (SolidBackgroundFillColor
/// Tertiary), for the colors below that must contrast with them.
const LIGHT_TEXT: Color = Color::rgb(0x1B, 0x1B, 0x1B);
const DARK_TEXT: Color = Color::rgb(0xFF, 0xFF, 0xFF);
const LIGHT_RIBBON: Color = Color::rgb(0xF9, 0xF9, 0xF9);
const DARK_RIBBON: Color = Color::rgb(0x28, 0x28, 0x28);

/// The colors the app chooses itself for the current theme and accent.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Palette {
    pub dark: bool,
    /// The Windows accent color.
    pub band: Color,
    /// What the title and tabs sit on: the Mica backdrop's tone.
    pub band_seen: Color,
    /// The fill behind the title and tabs: none, so Mica shows through.
    pub band_fill: Color,
    /// Text and icons in the title and tabs: the theme's text color.
    pub on_band: Color,
    pub band_hover: Color,
    pub band_pressed: Color,
    /// The selected tab's label, on the ribbon-colored tab.
    pub selected_tab: Color,
    /// A ribbon button whose formatting is on at the caret.
    pub active: Color,
    pub active_border: Color,
}

impl Palette {
    pub fn new(dark: bool, accent: Color) -> Self {
        let (ribbon, text) = if dark {
            (DARK_RIBBON, DARK_TEXT)
        } else {
            (LIGHT_RIBBON, LIGHT_TEXT)
        };
        // The accent reads on the ribbon when it is dark on a light ribbon or light on a dark one.
        let selected_tab = if is_light(accent) == dark {
            accent
        } else {
            text
        };
        Self {
            dark,
            band: accent,
            // No color of its own: Mica shows through the title and tabs, as it does the
            // rest of the window, and they take the theme's text color.
            band_seen: if dark { DARK_MICA } else { LIGHT_MICA },
            band_fill: Color::transparent(),
            on_band: text,
            // Washes of the text color, laid over the Mica.
            band_hover: with_alpha(text, 0x14),
            band_pressed: with_alpha(text, 0x22),
            selected_tab,
            active: mix(accent, ribbon, if dark { 0.65 } else { 0.8 }),
            active_border: mix(accent, ribbon, if dark { 0.35 } else { 0.55 }),
        }
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self::new(false, DEFAULT_ACCENT)
    }
}

thread_local! {
    static CURRENT: Cell<Palette> = Cell::new(Palette::default());
}

/// Makes `palette` the one the view's helper functions draw with. Set at the start of a render.
pub fn set_current(palette: Palette) {
    CURRENT.with(|current| current.set(palette));
}

pub fn current() -> Palette {
    CURRENT.with(Cell::get)
}

/// The user's Windows accent color (Settings › Personalization › Colors).
pub fn system_accent() -> Color {
    let read = || -> Result<Color> {
        let settings: native::UISettings =
            windows_core::factory::<native::UISettings, windows_core::imp::IGenericFactory>()?
                .ActivateInstance()?;
        let color = settings
            .cast::<native::IUISettings3>()?
            .GetColorValue(native::UIColorType::Accent)?;
        Ok(Color::rgb(color.r, color.g, color.b))
    };
    read().unwrap_or_else(|error| {
        crate::log::log!("could not read the accent color: {error:?}");
        DEFAULT_ACCENT
    })
}

/// Colors the window's minimize, maximize and close buttons to sit on the band. `element` is any
/// element in the window.
pub fn color_caption_buttons(element: &windows_core::IUnknown, palette: Palette) -> Result<()> {
    // XamlRoot is null until the element is in the window, so callers retry until this succeeds.
    let id = element
        .cast::<native::IUIElement>()?
        .XamlRoot()?
        .cast::<native::IXamlRoot2>()?
        .ContentIslandEnvironment()?
        .AppWindowId()?;
    let bar = native::AppWindow::GetFromWindowId(id)?.TitleBar()?;
    let color = |color: Color| {
        Some(native::Color {
            a: color.a,
            r: color.r,
            g: color.g,
            b: color.b,
        })
    };
    let clear = color(Color::transparent());
    let faded = mix(palette.band_seen, palette.on_band, 0.6);
    bar.SetButtonBackgroundColor(clear)?;
    bar.SetButtonInactiveBackgroundColor(clear)?;
    bar.SetButtonForegroundColor(color(palette.on_band))?;
    bar.SetButtonInactiveForegroundColor(color(faded))?;
    bar.SetButtonHoverBackgroundColor(color(palette.band_hover))?;
    bar.SetButtonHoverForegroundColor(color(palette.on_band))?;
    bar.SetButtonPressedBackgroundColor(color(palette.band_pressed))?;
    bar.SetButtonPressedForegroundColor(color(palette.on_band))
}

/// The window (HWND) that `element` is in. Its AppWindow id is the HWND's value.
pub fn window_handle(element: &windows_core::IUnknown) -> Result<isize> {
    let id = element
        .cast::<native::IUIElement>()?
        .XamlRoot()?
        .cast::<native::IXamlRoot2>()?
        .ContentIslandEnvironment()?
        .AppWindowId()?;
    Ok(id.value as isize)
}

/// Whether dark text reads better than white on `color` (the YIQ brightness test).
fn is_light(color: Color) -> bool {
    let brightness = u32::from(color.r) * 299 + u32::from(color.g) * 587 + u32::from(color.b) * 114;
    brightness / 1000 > 150
}

/// `color` with the given opacity (0 clear, 255 solid).
const fn with_alpha(color: Color, alpha: u8) -> Color {
    Color::argb(alpha, color.r, color.g, color.b)
}

/// `from` moved `amount` (0–1) of the way towards `to`.
fn mix(from: Color, to: Color, amount: f64) -> Color {
    let channel =
        |a: u8, b: u8| (f64::from(a) + (f64::from(b) - f64::from(a)) * amount).round() as u8;
    Color::rgb(
        channel(from.r, to.r),
        channel(from.g, to.g),
        channel(from.b, to.b),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_title_band_is_uncolored_with_the_theme_text_color() {
        let light = Palette::new(false, DEFAULT_ACCENT);
        assert_eq!(light.band_fill, Color::transparent());
        assert_eq!(light.on_band, LIGHT_TEXT);
        assert_eq!(Palette::new(true, DEFAULT_ACCENT).on_band, DARK_TEXT);
    }

    #[test]
    fn the_selected_tab_uses_the_accent_only_where_it_reads() {
        // Default blue on a light ribbon reads; on a dark ribbon it is too dark.
        assert_eq!(
            Palette::new(false, DEFAULT_ACCENT).selected_tab,
            DEFAULT_ACCENT
        );
        assert_eq!(Palette::new(true, DEFAULT_ACCENT).selected_tab, DARK_TEXT);
    }
}
