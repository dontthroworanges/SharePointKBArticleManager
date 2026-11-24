//! What the app remembers between sessions: the window's place and size, and the choices made
//! in the View tab. Kept as `key=value` lines in
//! `%APPDATA%\SharePoint Article Manager\settings.txt`.

use std::path::PathBuf;

/// Where the window was and whether it was maximized, in the terms of Win32's
/// GetWindowPlacement: the restored (normal) rectangle, in workspace pixels.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Placement {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub maximized: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Settings {
    pub placement: Option<Placement>,
    pub theme: Option<String>,
    pub ribbon: Option<String>,
    pub dark_page: bool,
    pub explorer_width: Option<f64>,
}

impl Settings {
    pub fn load() -> Self {
        // Settings saved before the app was renamed are read until they are next saved.
        [path(), old_path()]
            .into_iter()
            .flatten()
            .find_map(|path| std::fs::read_to_string(path).ok())
            .map(|text| Self::parse(&text))
            .unwrap_or_default()
    }

    pub fn save(&self) {
        let Some(path) = path() else { return };
        if let Some(folder) = path.parent() {
            _ = std::fs::create_dir_all(folder);
        }
        if let Err(error) = std::fs::write(&path, self.to_text()) {
            crate::log::log!("could not save settings: {error}");
        }
    }

    fn parse(text: &str) -> Self {
        let mut settings = Self::default();
        for line in text.lines() {
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            let value = value.trim();
            match key.trim() {
                "window" => {
                    let numbers: Vec<i32> =
                        value.split(',').filter_map(|n| n.trim().parse().ok()).collect();
                    if let [left, top, right, bottom, maximized] = numbers[..]
                        && right > left
                        && bottom > top
                    {
                        settings.placement = Some(Placement {
                            left,
                            top,
                            right,
                            bottom,
                            maximized: maximized != 0,
                        });
                    }
                }
                "theme" => settings.theme = Some(value.to_string()),
                "ribbon" => settings.ribbon = Some(value.to_string()),
                "dark_page" => settings.dark_page = value == "1",
                "explorer_width" => settings.explorer_width = value.parse().ok(),
                _ => {}
            }
        }
        settings
    }

    fn to_text(&self) -> String {
        let mut text = String::new();
        if let Some(p) = self.placement {
            text += &format!(
                "window={},{},{},{},{}\n",
                p.left,
                p.top,
                p.right,
                p.bottom,
                u8::from(p.maximized)
            );
        }
        if let Some(theme) = &self.theme {
            text += &format!("theme={theme}\n");
        }
        if let Some(ribbon) = &self.ribbon {
            text += &format!("ribbon={ribbon}\n");
        }
        text += &format!("dark_page={}\n", u8::from(self.dark_page));
        if let Some(width) = self.explorer_width {
            text += &format!("explorer_width={width}\n");
        }
        text
    }
}

fn path() -> Option<PathBuf> {
    let folder = std::env::var_os("APPDATA")?;
    Some(PathBuf::from(folder).join(super::APP_NAME).join("settings.txt"))
}

/// Where settings were kept when the app was called KB Page Editor.
fn old_path() -> Option<PathBuf> {
    let folder = std::env::var_os("APPDATA")?;
    Some(PathBuf::from(folder).join("KB Page Editor").join("settings.txt"))
}

#[repr(C)]
#[derive(Default)]
struct WindowPlacement {
    length: u32,
    flags: u32,
    show: u32,
    min_position: [i32; 2],
    max_position: [i32; 2],
    normal: [i32; 4],
}

#[link(name = "user32")]
unsafe extern "system" {
    fn GetWindowPlacement(window: isize, placement: *mut WindowPlacement) -> i32;
    fn SetWindowPlacement(window: isize, placement: *const WindowPlacement) -> i32;
}

/// From winuser.h.
const SW_SHOWNORMAL: u32 = 1;
const SW_SHOWMAXIMIZED: u32 = 3;

/// Where `window` is now, or None if Windows cannot say.
pub fn placement(window: isize) -> Option<Placement> {
    let mut placement = WindowPlacement {
        length: size_of::<WindowPlacement>() as u32,
        ..Default::default()
    };
    // SAFETY: `placement` is a WINDOWPLACEMENT with its length set, as the call requires.
    if unsafe { GetWindowPlacement(window, &mut placement) } == 0 {
        return None;
    }
    let [left, top, right, bottom] = placement.normal;
    Some(Placement {
        left,
        top,
        right,
        bottom,
        maximized: placement.show == SW_SHOWMAXIMIZED,
    })
}

/// Moves `window` back to where it was. Windows moves it onto a screen if that screen is gone.
pub fn restore(window: isize, saved: Placement) {
    let placement = WindowPlacement {
        length: size_of::<WindowPlacement>() as u32,
        flags: 0,
        show: if saved.maximized {
            SW_SHOWMAXIMIZED
        } else {
            SW_SHOWNORMAL
        },
        min_position: [-1, -1],
        max_position: [-1, -1],
        normal: [saved.left, saved.top, saved.right, saved.bottom],
    };
    // SAFETY: as above; the window is the app's own.
    if unsafe { SetWindowPlacement(window, &placement) } == 0 {
        crate::log::log!("could not restore the window placement");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_survive_a_round_trip() {
        let settings = Settings {
            placement: Some(Placement {
                left: -1800,
                top: 40,
                right: -200,
                bottom: 1000,
                maximized: true,
            }),
            theme: Some("Dark".into()),
            ribbon: Some("Classic Ribbon".into()),
            dark_page: true,
            explorer_width: Some(360.0),
        };
        assert_eq!(Settings::parse(&settings.to_text()), settings);
    }

    #[test]
    fn a_damaged_file_gives_defaults() {
        let settings = Settings::parse("window=1,2\nnonsense\nexplorer_width=wide");
        assert_eq!(settings, Settings::default());
    }
}
