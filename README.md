# SharePoint Article Manager

A native Windows editor for SharePoint Knowledge Base pages, written in Rust with WinUI 3. It works like Word, has a Markdown source view, and saves `.md` files that you import with **New > Import** in a SharePoint knowledge base.

## Build and run

Requirements:
- Windows 10 or 11
- Rust 1.95 or later (`winget install Rustlang.Rustup`)
- Visual Studio 2022 Build Tools with the C++ workload

```powershell
cargo run --release                       # empty page
cargo run --release -- samples\all-features.md
cargo test                                # Markdown round-trip tests
```

The first build downloads the Windows App SDK runtime (2.4.0) and copies it next to the executable. The app runs self-contained, so it needs no installer or MSIX package.

Set `KB_EDITOR_LOG=<file>` to write a log of editor events for debugging.

## What it supports

Every element from Microsoft's [Create and edit a SharePoint Knowledge Base manually](https://support.microsoft.com/en-us/sharepoint/knowledge-base/create-edit-knowledge-base-manually) page:

| Feature | Where in the app |
| --- | --- |
| Headings 1–6, Paragraph, Quote | Home › Styles box and right-click menu (each style shown in its own look, as in the SharePoint web editor), Quote button, Ctrl+Alt+1–6, Ctrl+Shift+N |
| Bold, italic, strikethrough, `==highlight==` | Home, Ctrl+B / Ctrl+I / Ctrl+Shift+X / Ctrl+Shift+H |
| Inline code, inline math `$…$` | Home, Ctrl+Alt+C |
| Bulleted, numbered and task lists, nesting. Enter starts the next item of the same kind (Enter on an empty item ends the list); wrapped lines line up with the item's text | Home, Ctrl+Shift+L, Tab / Shift+Tab |
| Links and `[[Page name]]` links | Home or Insert, Ctrl+K |
| Images and videos (`:::image …:::`) | Insert › Picture / Video |
| Tables | Insert › Table (cells accept inline Markdown) |
| Code blocks, Mermaid diagrams | Insert › Code block / Diagram |
| Display math `$$ … $$` | Insert › Equation |
| Horizontal rule | Insert › Divider |
| Properties (YAML front matter) | View › Properties |
| Markdown source | View › Markdown, or the Visual / Markdown buttons beside the zoom in the status bar |
| Zoom 10–500% | Status bar slider, View › Zoom, Ctrl++ / Ctrl+- / Ctrl+0 |
| Ribbon layouts | Chevron at the end of the ribbon: Single Line Ribbon (one row of small buttons) or Classic Ribbon (groups with large buttons and group names), as in Word for the web |
| Narrow windows | Ribbon commands (whole groups in the Classic ribbon) that do not fit move into a “More” (…) menu on every tab |
| Light and dark themes, Mica | View › Theme: follow Windows, Light or Dark. The window uses the Mica backdrop; the page stays white in every theme (unless View › Switch Background is on). The title and tabs sit on Mica too, with no color of their own; the selected tab is underlined in the Windows accent color |
| SharePoint look | The page uses SharePoint's modern-page type: Segoe UI, 18px body text, 28/24/20/18px semibold headings, #323130 text and #0078D4 links |
| Dark page | View › Switch Background, as in Word: the page goes dark. The saved file is unchanged |
| Remembered between sessions | Window position and size (and whether it was maximized), theme, ribbon layout, Switch Background and the Document Editor's width, in `%APPDATA%\SharePoint Article Manager\settings.txt` (settings saved under the app's old name, KB Page Editor, are picked up) |
| Title bar | The app's name and version at the left; the open document's name centered, followed by "(Unsaved)" when it has changes. File › Save is greyed out while the file is up to date |
| Right-click menu | Cut, Copy, Paste, Select All (no Bold / Italic row above them); with text selected, the Home tab's Font and Paragraph commands; then the styles. No command popup appears over selected text |
| About | File › About: version, description and a link to the project on GitHub |
| Document Editor: Navigation (heading outline: click to jump, collapse), Find (click a result to go to it), Replace, Spelling; drag its right edge to resize (tools that no longer fit move into a "…" menu) | View › Navigation / Find / Replace / Spelling (each opens its tool; click again to close), Ctrl+F, Ctrl+H |
| Spell check | Windows spell checker: red squiggles in the page; right-click › Proofing for suggestions, Add to dictionary, Ignore. Document Editor › Spelling lists every misspelling (not in code or math) with suggestions: Change, Change all (that word only), Ignore, Ignore all, Apply all |

### Standard editing keys

| Action | Keys |
| --- | --- |
| Undo / redo (a paste and its clean-up undo as one step) | Ctrl+Z / Ctrl+Y or Ctrl+Shift+Z |
| Cut, copy, paste, select all | Ctrl+X, Ctrl+C, Ctrl+V, Ctrl+A (also Shift+Del, Ctrl+Ins, Shift+Ins) |
| Delete or move by word, start/end of line or page | Ctrl+Backspace / Ctrl+Del, Ctrl+←/→, Home/End, Ctrl+Home/End |
| New, open, save, save as | Ctrl+N, Ctrl+O or Ctrl+F12, Ctrl+S or Shift+F12, Ctrl+Shift+S or F12 |
| Find, replace, next / previous result | Ctrl+F, Ctrl+H, F3 / Shift+F3 |
| Line break within a paragraph | Shift+Enter |

Keys that would apply formatting Markdown can't store do nothing: underline (Ctrl+U), alignment (Ctrl+L/E/R/J), line spacing (Ctrl+1/2/5), font size (Ctrl+[ ], Ctrl+Shift+< >) and all caps (Ctrl+Shift+A). Ctrl+= is zoom rather than subscript.

If a file contains Markdown the editor doesn't model (HTML, footnotes, list items with several paragraphs), the editor keeps it exactly as written and shows it in a "kept as written" card. The aim is that opening and saving a file without edits leaves it unchanged; this is verified for `samples/all-features.md` and by the tests in `src/markdown/tests.rs`.

## How it is built

```
src/
  main.rs            entry point; opens the file passed on the command line
  markdown/          Markdown ↔ document model (comrak), with round-trip tests
  rich.rs            document model ↔ RichEditBox, through the text object model (TOM)
  app/mod.rs         editor state, commands, files, dialogs (windows-reactor component)
  app/view.rs        ribbon, page, properties pane, status bar
  app/view/ribbon.rs each tab's commands as named groups, drawn as the single-line or Classic ribbon
  app/host.rs        attaches each native RichEditBox: content, events, Word shortcuts
  app/spelling.rs    the Windows spell checker, for the Spelling pane
  app/settings.rs    what is remembered between sessions (window placement, View choices)
  app/styles_box.rs  the Styles box list, each style in its own look
  app/view/icons.rs  Fluent UI icons as XAML path markup, generated by tools/fluent-icons.ps1
  app/theme.rs       light and dark themes, accent colors, the page's theme
  winui/bindings.rs  generated WinUI bindings (Microsoft.UI.Text and a few others)
tools/bindgen/       regenerates src/winui/bindings.rs (see filter.txt)
vendor/windows-reactor/  windows-reactor 0.100 with the patches listed below
```

**Page structure.** The page is a column of segments. Each run of paragraphs, headings, quotes and lists is one borderless `RichEditBox`, so selection, typing, undo and spell check behave like Word within it. Tables, pictures, code, equations and dividers are native controls placed between those text segments. Inserting an object splits the text segment at the caret.

**Format conventions inside the editor** (see the top of `rich.rs`):
- Headings use the TOM heading styles.
- Quotes use a right indent.
- List nesting uses the left indent.
- Tasks start with a ☐/🗹 glyph.
- Code and math are tagged with pseudo-locale language tags (`qps-ploc`, `qps-plocm`). WinUI resets font names after loading, so the tags are what identify these runs.

### Patches to windows-reactor

`windows-reactor` 0.100 is a preview. A patched copy lives in `vendor/windows-reactor` and is wired in through `[patch.crates-io]`. Each change is marked `kb-editor patch`:

1. **`ElementRef::request_native_object`**: returns the native WinUI element. Reactor's `RichEditBox` only exposes plain text, and the editor needs the text object model.
2. **`ElementRef::request_resource_overrides`**: applies theme resources to any element. Only `Button` has these upstream. The app uses them to make editors borderless.
3. **Uncontrolled `RichEditBox`**: without `text` or `on_text_changed`, Reactor no longer watches the document and resets it to empty.
4. **`set_window_close_guard`**: lets the app cancel a window close so it can ask "Save changes?".
5. **Menu placement**: menus attached to a button open below it, like a menu bar, instead of above it (the WinUI default).
6. **More theme brushes**: `ThemeBrush` gains `SecondaryText`, `LayerBackground`, `Divider` and `Layer`, so the window chrome can follow light and dark themes on Mica.

These patches are candidates to upstream to [microsoft/windows-rs](https://github.com/microsoft/windows-rs).

## Known limitations of this version

- Mermaid diagrams and equations are edited as source text. There is no rendered preview yet; the candidate pure-Rust renderers are listed in RESEARCH.md.
- Table cells are plain text boxes that hold inline Markdown (for example `**bold**`), not rich text.
- A text selection can't span a table, picture or other object.
- Task checkboxes are toggled with **Home › Mark task done**, not by clicking the glyph. With several tasks selected, it marks them all done (or all not done, if they all were).
- The status bar has no messages; problems (a file that could not be saved, for example) show in a bar above the page until closed.
- Pasting from Word keeps only formatting that Markdown supports. The page is cleaned up after a paste; one Ctrl+Z undoes the paste and the clean-up together.
- Dialogs follow the Windows theme, so they're dark in dark mode while the page stays light (unless View › Switch Background is on).
- A change of Windows accent color shows after the next theme change or restart.
- Space before and after paragraphs can't be set: Markdown has no way to store it, and SharePoint applies its own spacing when it shows the page.
- There is no direct upload to SharePoint yet. Save the file, then use **New > Import** in the knowledge base.
- The colors and indents that mark headings, quotes and lists live in the editor only. The saved file is plain Markdown.
