# SharePoint Article Manager

A native Windows editor for SharePoint Knowledge Base pages, written in Rust with WinUI 3. It works like Word, has a Markdown source view, and saves `.md` files that you import with **New > Import** in a SharePoint knowledge base.

## Build and run

Requirements:
- Windows 10 or 11
- Rust 1.95 or later (`winget install Rustlang.Rustup`)
- Visual Studio 2022 Build Tools with the C++ workload
- The Microsoft Edge WebView2 Runtime, which the page runs in (part of Windows 11; on Windows 10 it comes with Edge, or from Microsoft's WebView2 download page)

```powershell
cargo run --release                       # empty page
cargo run --release -- samples\all-features.md
cargo test                                # Markdown and page round-trip tests
```

The first build downloads the Windows App SDK runtime (2.4.0) and copies it next to the executable. The app runs self-contained, so it needs no installer or MSIX package.

Set `KB_EDITOR_LOG=<file>` to write a log of editor events for debugging (it includes the page's script errors, and enables the page's developer tools: right-click › Inspect).

The page's files are in `web/`. A debug build serves them straight from there, so a change to `editor.js` or `editor.css` shows on the next start without a rebuild; a release build carries them inside the exe. `web/vendor/tiptap.js` is built from `web/package.json` (`npm ci`, then `npm run build`, in `web/`) and checked in, so building the app does not need Node.js.

## What it supports

Every element from Microsoft's [Create and edit a SharePoint Knowledge Base manually](https://support.microsoft.com/en-us/sharepoint/knowledge-base/create-edit-knowledge-base-manually) page:

| Feature | Where in the app |
| --- | --- |
| Headings 1–6, Paragraph, Quote | Home › Styles box and right-click menu (each style shown in its own look, as in the SharePoint web editor), Quote button, Ctrl+Alt+1–6, Ctrl+Shift+N |
| Bold, italic, strikethrough, `==highlight==` | Home, Ctrl+B / Ctrl+I / Ctrl+Shift+X / Ctrl+Shift+H |
| Inline code, inline math `$…$` | Home, Ctrl+Alt+C |
| Bulleted, numbered and task lists, nesting. Enter starts the next item of the same kind (Enter on an empty item ends the list); wrapped lines line up with the item's text | Home, Ctrl+Shift+L, Tab / Shift+Tab |
| Links and `[[Page name]]` links | Home or Insert, Ctrl+K. Hovering a link shows its address at the bottom of the page; Ctrl+click opens it in the browser |
| Images and videos (`:::image …:::`) | Insert › Picture / Video. On the page, Edit… (or a double-click) changes the address and alt text |
| Tables | Insert › Table. Cells hold formatted text; while the cursor is in a table, a bar above it adds and removes rows and columns, or removes the table. Tab moves to the next cell |
| Code blocks, Mermaid diagrams | Insert › Code block / Diagram |
| Display math `$$ … $$` | Insert › Equation |
| Horizontal rule | Insert › Divider |
| Properties (YAML front matter) | View › Properties |
| Markdown source | View › Markdown, or the Visual / Markdown buttons beside the zoom in the status bar |
| Zoom 10–500% | Status bar slider, View › Zoom, Ctrl++ / Ctrl+- / Ctrl+0, Ctrl+mouse wheel over the page |
| Page Width | View › Zoom › Page Width: the page as wide as the window leaves room for, at 100% |
| Ribbon layouts | Chevron at the end of the ribbon: Single Line Ribbon (one row of small buttons) or Classic Ribbon (groups with large buttons and group names), as in Word for the web |
| Narrow windows | Ribbon commands (whole groups in the Classic ribbon) that do not fit move into a “More” (…) menu on every tab |
| Light and dark themes, Mica | View › Theme: follow Windows, Light or Dark. The window uses the Mica backdrop; the page stays white in every theme (unless View › Switch Background is on). The title and tabs sit on Mica too, with no color of their own; the selected tab is underlined in the Windows accent color |
| SharePoint look | The page uses SharePoint's modern-page type: Segoe UI, 14px body text (14, 16 or 18px in Settings › Options), 28/24/20/18px semibold headings with a rule under Heading 1 and 2, #323130 text and #0078D4 links |
| Dark page | View › Switch Background, as in Word: the page goes dark. The saved file is unchanged |
| Remembered between sessions | Window position and size (and whether it was maximized), theme, ribbon layout, Switch Background, the Document Editor's width and the choices in Settings (including Always show the Document Editor), in `%APPDATA%\SharePoint Article Manager\settings.txt` (settings saved under the app's old name, KB Page Editor, are picked up) |
| Title bar | The app's name and version at the left; the open document's name centered, followed by "(Unsaved)" when it has changes. File › Save is greyed out while the file is up to date. File is a ribbon tab (New page, Open, Rename, Save, Save as) like Home, Insert and View. File › Rename renames the open file in its folder, in a dialog; without an extension the name keeps the file's own (.md) |
| Right-click menu | The page's own menu: spelling suggestions, Cut, Copy, Paste and Select All |
| Help | Help › About: opens Settings › About (version, description and a link to the project on GitHub). Help › Microsoft Resources: opens Microsoft's guide to creating and editing knowledge base articles in the browser |
| Settings | The gear at the right end of the tabs shows Settings over the whole window below the title bar, as in Notepad; the back arrow at the top left of the title bar returns to the page. Categories run down the left, as in Windows 11 Settings: Options and About. Options › Always show the Document Editor opens the Document Editor pane at every start (it can still be closed while working). Options › Show heading styles in Navigation adds each heading's style, such as (H2), in grey after its name in the Navigation pane. Options › Space below the cursor keeps blank page below the line being typed (none, a quarter, a third or half the window; a third by default) |
| Document Editor: Navigation (heading outline: click to jump, collapse), Find (click a result to go to it), Replace, Spelling; drag its right edge to resize (tools that no longer fit move into a "…" menu) | View › Navigation / Find / Replace / Spelling (each opens its tool; click again to close), Ctrl+F, Ctrl+H |
| Spell check | Windows spell checker: red squiggles in the page; right-click for suggestions and Add to dictionary. Document Editor › Spelling lists every misspelling (not in code or math) with suggestions: Change, Change all (that word only), Ignore, Ignore all, Apply all |

### Standard editing keys

| Action | Keys |
| --- | --- |
| Undo / redo | Ctrl+Z / Ctrl+Y or Ctrl+Shift+Z |
| Cut, copy, paste, select all | Ctrl+X, Ctrl+C, Ctrl+V, Ctrl+A (also Shift+Del, Ctrl+Ins, Shift+Ins) |
| Delete or move by word, start/end of line or page | Ctrl+Backspace / Ctrl+Del, Ctrl+←/→, Home/End, Ctrl+Home/End |
| New, open, save, save as | Ctrl+N, Ctrl+O or Ctrl+F12, Ctrl+S or Shift+F12, Ctrl+Shift+S or F12 |
| Find, replace, next / previous result | Ctrl+F, Ctrl+H, F3 / Shift+F3 |
| Line break within a paragraph | Shift+Enter |

Keys that would apply formatting Markdown can't store do nothing, such as underline (Ctrl+U). Ctrl+= is zoom rather than subscript.

If a file contains Markdown the editor doesn't model (HTML, footnotes, list items with several paragraphs), the editor keeps it exactly as written and shows it in a "kept as written" card. The aim is that opening and saving a file without edits leaves it unchanged; this is verified for every file in `samples/` by the tests in `src/bridge.rs` and `src/markdown/tests.rs`, and through the running app by `KB_SELF_TEST` (below).

## How it is built

```
src/
  main.rs            entry point; opens the file passed on the command line
  markdown/          Markdown ↔ document model (comrak), with round-trip tests
  bridge.rs          document model ↔ the page's document (ProseMirror JSON), with round-trip tests
  app/mod.rs         editor state, commands, files, dialogs (windows-reactor component)
  app/page.rs        the page's WebView2: its settings, files and message channel
  app/keys.rs        window-wide shortcuts while focus is in the ribbon or the panes
  app/view.rs        ribbon, page, properties pane, status bar
  app/view/ribbon.rs each tab's commands as named groups, drawn as the single-line or Classic ribbon
  app/spelling.rs    the Windows spell checker, for the Spelling pane
  app/settings.rs    what is remembered between sessions (window placement, View choices)
  app/view/settings_page.rs Settings: Options and About (gear at the right of the tabs)
  app/styles_box.rs  the Styles box list, each style in its own look
  app/view/icons.rs  Fluent UI icons as XAML path markup, generated by tools/fluent-icons.ps1
  app/theme.rs       light and dark themes, accent colors
  winui/bindings.rs  generated WinUI bindings (a few Microsoft.UI.Xaml types, the spell checker)
  webview/bindings.rs generated WebView2 bindings
web/
  index.html, editor.css, editor.js   the page: Tiptap set up for the SharePoint dialect
  bundle-entry.js, package.json       what goes into vendor/tiptap.js (npm run build)
assets/              the app icon: Fluent UI's color "Book Database" (MIT, see LICENSE-fluentui-system-icons),
                     as SVG, the exe/window .ico (embedded by build.rs) and PNGs for the title bar and About
tools/bindgen/       regenerates src/winui/bindings.rs (see filter.txt)
tools/webview-bindgen/ regenerates src/webview/bindings.rs (see filter.txt)
vendor/windows-reactor/  windows-reactor 0.100 with the patches listed below
```

**The page.** The page is one [Tiptap](https://tiptap.dev) editor (built on ProseMirror) in a WebView2, laid out as SharePoint shows the article. The app never hands it Markdown: it parses the file with its own parser (`src/markdown`), sends the page a document tree (`src/bridge.rs`), and turns the tree it gets back into Markdown with its own serializer, so the file keeps the SharePoint dialect exactly. Front matter stays on the app's side. Parts of the dialect with no editor of their own travel as blocks that keep their source: equations and Markdown kept as written are edited as text on a card; pictures and videos keep every attribute of their `:::image:::` line.

The app and the page talk in JSON messages, listed at the top of `web/editor.js`. The app sends commands (from the ribbon, the dialogs and the panes), documents and view settings (zoom, page width, text size, dark page, space below the cursor); the page reports edits, the formatting at the cursor, its headings, search results and its prose for the spell checker. Keys typed in the page stay in the page (WebView2 keeps them), so it passes the app's shortcuts (files, Find, zoom, Ctrl+K) on.

The Markdown view is a plain text box on the same page. Switching to it sends the page's Markdown; switching back rebuilds the visual page only if the Markdown changed.

**Checking it.** `KB_SELF_TEST=<out.md>` opens the file given on the command line, switches to the Markdown view and back, checks spelling and saves to `out.md`, then quits; with `KB_EDITOR_LOG` set, each step is logged. `KB_SELF_TEST_QUERY=<text>` also searches for the text and makes the first result bold; `KB_SELF_TEST_FIX=1` applies every spelling suggestion before saving.

### Patches to windows-reactor

`windows-reactor` 0.100 is a preview. A patched copy lives in `vendor/windows-reactor` and is wired in through `[patch.crates-io]`. Each change is marked `kb-editor patch`:

1. **`ElementRef::request_native_object`**: returns the native WinUI element. The app uses it for the window, the Styles box and focusing the page.
2. **`ElementRef::request_resource_overrides`**: applies theme resources to any element. Only `Button` has these upstream.
3. **Uncontrolled `RichEditBox`**: without `text` or `on_text_changed`, Reactor no longer watches the document and resets it to empty. (Used by the RichEdit version of the editor, kept on the `archive/richedit-editor` branch.)
4. **`set_window_close_guard`**: lets the app cancel a window close so it can ask "Save changes?".
5. **Menu placement**: menus attached to a button open below it, like a menu bar, instead of above it (the WinUI default).
6. **More theme brushes**: `ThemeBrush` gains `SecondaryText`, `LayerBackground`, `Divider` and `Layer`, so the window chrome can follow light and dark themes on Mica.

These patches are candidates to upstream to [microsoft/windows-rs](https://github.com/microsoft/windows-rs).

## Known limitations of this version

- Mermaid diagrams and equations are edited as source text. There is no rendered preview yet.
- The right-click menu is the page's own: it has the spelling suggestions, but not the paragraph styles the RichEdit version added to it.
- Table column alignment (`:---:`) is kept but not shown, and a column added on the page has none.
- Undo in the Markdown view covers typing only; a replacement made from the Replace tab there can't be undone.
- The status bar has no messages; problems (a file that could not be saved, for example) show in a bar above the page until closed.
- Pasting keeps only formatting Markdown supports; anything else is dropped as it is pasted.
- Dialogs follow the Windows theme, so they're dark in dark mode while the page stays light (unless View › Switch Background is on).
- Space before and after paragraphs can't be set: Markdown has no way to store it, and SharePoint applies its own spacing when it shows the page.
- There is no direct upload to SharePoint yet. Save the file, then use **New > Import** in the knowledge base.
