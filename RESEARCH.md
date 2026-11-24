# SharePoint KB Markdown Editor — Research Notes

Research collected 2026-11-22 for a Windows desktop app that writes Markdown files for **SharePoint Knowledge Base (preview)**, built in **Rust** with **WinUI 3**. The editor is WYSIWYG first, with a code view second.

---

## 1. What SharePoint Knowledge Base is

- Announced in **Frontier preview** on 14 Oct 2026. Features and syntax may change before general availability.
- A knowledge base is a *specialized SharePoint library of interlinked Markdown (`.md`) files*, called "KB pages", on a site. It uses the site's permissions: members edit and visitors read.
- SharePoint has two editors that work on the same file: a **Visual editor** and a **Source editor**. Our app copies this model.
- Page properties are stored as **YAML front matter**.
- A knowledge base can be created blank, by importing **Markdown files or folders** (the folder hierarchy is kept), from a Confluence HTML export ZIP, or from Classic wiki pages.
- Writing, importing, reading, and searching pages by hand needs **no Copilot Credits**. Only the Copilot and Autopilot features use credits.

### Import behavior that affects our app

| Fact | What it means for the app |
| --- | --- |
| Importing copies the files once. There is no sync. | The app produces files. Uploading them is a separate step (see §6). |
| **Images referenced in Markdown are not copied** on import. | Warn the user about local image paths. Push them to use URLs that readers can reach, such as SharePoint-hosted assets. |
| Renaming another file type to `.md` doesn't convert it. | Always write real UTF-8 Markdown. |
| Renaming or moving a page can break `[[wiki links]]` that point to it. | Build a link checker and a "rename page and update links" command. |
| Subpages exist, and importing a folder keeps its hierarchy. | Treat a folder as a KB workspace (folder tree = page tree). |
| Add 10 or fewer files per Copilot request. | Applies only to Copilot. It doesn't limit imports. |

---

## 2. Syntax SharePoint supports (the toolbar must cover all of it)

Source: [Create and edit a SharePoint Knowledge Base manually](https://support.microsoft.com/en-us/sharepoint/knowledge-base/create-edit-knowledge-base-manually)

| # | Feature | Markdown | Proposed toolbar control |
| --- | --- | --- | --- |
| 1 | Headings H1–H6 | `# Heading` … `###### Heading` | Paragraph style dropdown (Normal, H1–H6) |
| 2 | Bold | `**Important**` | Toggle (Ctrl+B) |
| 3 | Italic | `*Emphasis*` | Toggle (Ctrl+I) |
| 4 | Strikethrough | `~~Old wording~~` | Toggle |
| 5 | Highlight | `==Review this==` | Toggle (**non-standard, not GFM**) |
| 6 | Bulleted list | `- First item` | Toggle |
| 7 | Numbered list | `1. First step` | Toggle |
| 8 | Task list | `- [ ] To do` / `- [x] Completed` | Toggle. Checkboxes can be clicked in WYSIWYG view |
| 9 | Quote | `> Quoted text` | Toggle |
| 10 | Inline code | `` `code` `` | Toggle |
| 11 | Code block | ```` ``` ```` fenced | Insert, with a language picker |
| 12 | Hyperlink | `[Microsoft Support](https://support.microsoft.com)` | Insert link dialog (Ctrl+K) |
| 13 | Wiki link | `[[Page name]]` | Insert page link (picks from the workspace's pages) |
| 14 | Image | `:::image type="content" source="url" alt-text="Description.":::` | Insert image dialog (source + alt text) |
| 15 | Video | Same `:::image type="content" …:::` syntax with a video URL | Insert video dialog |
| 16 | Table | GFM pipe table (header row, separator row, data rows) | Insert table, plus row and column commands |
| 17 | Math | `$x^2$` inline, `$$ … $$` block | Insert inline or block equation |
| 18 | Diagram | ` ```mermaid ` fenced block | Insert Mermaid diagram (with live preview) |
| 19 | Horizontal rule | `---` on its own line | Insert divider |
| 20 | Properties | YAML front matter between `---` lines at the top of the file | **Properties** pane: Add property, edit key/value |

Notes and unknowns:
- The documentation doesn't list keyboard shortcuts, file size limits, naming rules, or required front matter keys. Use the shortcuts that Word and SharePoint users already know.
- Images and videos use **Microsoft Learn–style `:::image` triple-colon syntax**, not `![alt](url)`. No standard Markdown library parses it, so it needs a custom extension (see §4).
- Whether SharePoint also accepts standard `![alt](url)` images, or `[[Page|label]]` wiki-link labels, is **not documented**. Before relying on either, test it by importing a sample file.
- `---` serves both as the front matter delimiter and as a horizontal rule. The serializer must emit a blank line before a body `---` so it is never read as front matter.

---

## 3. Rust + WinUI 3: current state (Nov 2026)

### Recommended: `windows-reactor` (official, Microsoft / windows-rs)
- Crate `windows-reactor` **0.100.0** (27 Oct 2026), license MIT/Apache-2.0, **preview** (the API may change). Minimum Rust version is **1.95**.
- It is a declarative, Elm-style API: a `Component` has `create`, `update(msg)`, and `view() -> View`. Reactor compares each new view with the current native WinUI tree and updates only the controls that changed.
- Every control this app needs is available: **CommandBar, AppBarButton, CommandBarFlyout, MenuBar, DropDownButton, SplitButton, ToggleButton-style buttons, TabView, NavigationView, TreeView, SplitView, TextBox, RichEditBox, ContentDialog, Flyout, TeachingTip, InfoBar, TitleBar, WebView2**.
- `windows-reactor-setup` stages the Windows App Runtime (bootstrapping for unpackaged apps).
- The `gallery` sample in windows-rs is a complete WinUI 3 app that builds to about a 3 MB binary. Run it with `cargo run -p gallery`.
- Related crates: `windows-webview` 0.100 (safe WebView2 wrapper with an optional `reactor` feature), `windows-canvas` (Direct2D), and `windows-composition`.
- Windows App SDK: the latest stable is **2.5.1** (Sept 2026).
- Known gap: the November release notes say there is still work to do on API selection and on sharing generated types between libraries.

### Alternatives
- **winui3-rs** (Alovchin91): experimental and unofficial bindings generated from metadata. It supports WinAppSDK 2.6/2.7 and gives lower-level imperative access. Use it only as a fallback.
- **waterui-winui**: a WinUI 3 backend for the WaterUI framework. It adds a dependency on a third-party framework.
- Raw `windows` crate plus hand-written XAML activation: possible, but you have to write a lot of code yourself.

---

## 4. The main design decision: how to build WYSIWYG

### Option A — Native `RichEditBox` (WinUI)
- Pros: fully native, with Fluent look and feel.
- Cons: RichEditBox is a RichText (RTF/TOM) control. It has **no tables, task-list checkboxes, Mermaid rendering, math rendering, or block-level custom nodes**. We would have to build a Markdown↔RTF round-trip, which loses information. **It can't cover features 8, 14–18 properly.** Not recommended.

### Option B — WinUI shell + WebView2-hosted editor (fastest to build, but not fully native)
- The window frame, CommandBar or toolbar, menus, file tree, Properties pane, dialogs, and settings are all **native WinUI** through `windows-reactor`.
- The editing surface is a `WebView2` control. It loads a local bundled page served through `set_virtual_host_name_to_folder_mapping`, so it works offline with no server.
- Native and web code talk through WebView2 web messages (JSON):
  - Toolbar click → Rust → `PostWebMessage {cmd:"toggleBold"}` → editor runs the command.
  - Editor selection change → `{state:{bold:true, heading:2, inTable:true…}}` → Rust updates the toggle states on the toolbar.
  - Save → editor returns the Markdown, and Rust writes the file.
- WYSIWYG editor candidates (both built on ProseMirror):
  - **Milkdown**: Markdown-first, built on remark. Good round-trip fidelity. Plugin system for custom syntax.
  - **TipTap**: larger ecosystem with tables, task lists, and math extensions. Its Markdown serialization is an add-on.
- Custom nodes and marks to build for SharePoint syntax: `==highlight==` mark, `[[wiki link]]` inline node, `:::image …:::` block node (image and video variants), math inline and block (render with **KaTeX**), Mermaid code block with live render (**mermaid.js**), and front matter (hidden in the editor and edited in the native Properties pane).
- **Code view**: **CodeMirror 6** in the same WebView, with Markdown syntax highlighting. Switching views passes the current Markdown string between the two. A native `TextBox` would also work but has no syntax highlighting.
- The **Markdown string is always the source of truth**. Switching to WYSIWYG parses it, and switching back serializes it. Test the round trip on every syntax element so the app never rewrites a user's Markdown in unexpected ways.

### Option C — Fully native block editor (no WebView2) ✅ *preferred if staying native*

The editor is a vertical list of **blocks**. Each Markdown block is shown by its own native WinUI control, similar to how Notion or Loop work. A single `RichEditBox` can't hold the whole document, but it works well for *one paragraph's* inline formatting.

| Markdown block | Native control |
| --- | --- |
| Paragraph / H1–H6 / quote | `RichEditBox`. The block type sets the font size or quote styling |
| Bold / italic / strike | RichEditBox `ITextCharacterFormat` (`Bold`, `Italic`, `Strikethrough`) |
| `==highlight==` | `ITextCharacterFormat.BackgroundColor` |
| Inline code | Character format with a monospace font and background |
| Links / `[[wiki links]]` | `ITextRange.Link` (styled differently for wiki links) |
| Bulleted / numbered list item | Indented row: bullet or number glyph + `RichEditBox` |
| Task item | `CheckBox` + `RichEditBox` |
| Table | `Grid` of `RichEditBox` cells, with row and column commands on a `CommandBarFlyout` |
| Code block | Monospace `RichEditBox`, colored by `syntect` (Rust), with a language `ComboBox` |
| Image | `Image` control showing the image, plus a flyout for source and alt text |
| Video | `MediaPlayerElement` (or a thumbnail card that opens the URL) |
| Math `$…$` / `$$…$$` | Rendered in Rust to SVG and shown with `Image` + `SvgImageSource`. Clicking it opens the LaTeX source for editing |
| Mermaid | Rendered in Rust to SVG and shown with `Image`. Uses a split view: source `TextBox` + live preview |
| Horizontal rule | `Border` / `Rectangle` |
| Front matter | Native Properties pane (not in the editor) |

**Pure-Rust renderers that replace JS (KaTeX/mermaid.js):**
- Math: **RaTeX** (pure Rust, covers more than 99.5% of KaTeX syntax, outputs SVG/PNG), **LaTeX-Rust**, or **rightkit_typeset** (mitex → Typst → SVG).
- Mermaid: **mermaid-rs-renderer**, **pele-rs** (33 diagram types, themeable SVG, built for editors), **mermaid-to-svg** (Warp, uses a dagre layout like mermaid.js), **rusty-mermaid**, **fermaid**. Their coverage and output quality differ. Test them against sample diagrams before choosing one, and keep a fallback that shows the source code block when rendering fails.

**Data model:** Rust owns a document model: `Vec<Block>`, where each block holds a list of inline runs (text + marks). This works like ProseMirror, but in Rust.
- `comrak` AST → model, which builds the blocks.
- Model → Markdown serializer, used when saving and when switching to code view.
- Inline runs ↔ RichEditBox: after an edit, read the TOM ranges back and rebuild the runs.

**Code view:** a native monospace `RichEditBox` colored by `syntect` or `tree-sitter-md`, or a plain `TextBox` for v1.

**Work the block approach adds (budget for it):**
- Keyboard behavior across blocks:
  - Enter at the end of a block splits it.
  - Backspace at the start merges with the block above.
  - Up/Down arrows move between blocks.
  - Typing Markdown shortcuts converts the block (`# ` → heading, `- [ ] ` → task).
- Selection across multiple blocks for copy and delete. Hard to do across separate controls. v1 can limit selection to one block and add a whole-block selection mode.
- One undo stack for the whole document (each RichEditBox has its own undo by default, which has to be turned off or redirected).
- Copy/paste: put Markdown and plain text on the clipboard, and convert pasted HTML or RTF.
- Accessibility comes mostly from the native controls, which is better than a WebView.
- Performance: use `ItemsRepeater` / `ListView` virtualization for long pages.

### Option D — Custom-drawn editor (Direct2D + DirectWrite)
Draw the whole editing surface yourself with `windows-canvas` (Direct2D) and DirectWrite, the way code editors like Zed do. This gives full control and seamless selection across blocks. The costs are that you must build caret handling, IME via Text Services Framework, accessibility (UI Automation providers), and hit-testing yourself. **It takes the most work.** Consider it only if Option C's selection across blocks turns out to be a dealbreaker.

### Option C′ — "Text flow + objects" hybrid (Word-like) ✅ **recommended for Word users**

The target users know Word, and in Word you can select text freely across paragraphs, headings and lists. Option C, with one box per paragraph, breaks that habit. Instead, group blocks the way Word does:

- **Text flow segments:** consecutive paragraphs, headings, quotes, bulleted and numbered lists live in **one `RichEditBox`**.
  - RichEditBox (TOM) supports paragraph-level formatting: `ITextParagraphFormat.ListType` for bullets and numbering, indent levels, and named styles for H1–H6 and quotes.
  - Clicking, dragging, Shift+arrow, Ctrl+A, undo and spell check therefore work across the whole segment exactly as in Word.
- **Object segments:** tables, code blocks, images and videos, block math, Mermaid diagrams and horizontal rules are separate controls placed between flow segments. Word users already treat tables and pictures as objects with their own selection behavior, so this feels familiar.
- **Task lists:** use a list with a ☐/☑ marker character, and toggle it when the marker is clicked (hit-test with `GetRangeFromPoint`). *Prototype this early. Custom TOM list markers need verification in WinUI's RichEditBox.*
- **Inline math** (`$x^2$`): an inline object in the RichEditBox (`InsertImage` of the rendered SVG/PNG), or styled text in v1.
- Seams remain only at object boundaries (for example, selecting from a paragraph through a table), which Word users also find awkward in Word.
- **Hardest part:** mapping RichEditBox paragraph and character formatting to and from the Markdown model, both reliably and without losing information. Build round-trip tests first.

### Word-familiar UX checklist
- A ribbon-style toolbar: **Home / Insert** tabs (`SelectorBar` + `CommandBar`), with a **Styles gallery** (Normal, Heading 1–6, Quote, Code) like Word's.
- Word keyboard shortcuts:
  - Ctrl+B / Ctrl+I / Ctrl+K for bold, italic and link.
  - Ctrl+Alt+1–3 for headings.
  - Ctrl+Shift+L for a bulleted list.
  - Tab / Shift+Tab to indent and outdent list items.
  - Ctrl+Z / Ctrl+Y for undo and redo.
  - Ctrl+F / Ctrl+H for find and replace.
- Word-style AutoFormat as you type:
  - `1. ` starts a numbered list.
  - `- ` or `* ` starts a bulleted list.
  - `---` inserts a divider.
- **Paste from Word:** read RTF/HTML from the clipboard and keep only what Markdown supports (bold, italic, strike, highlight, headings, lists, tables, links). Show a short notice when formatting was dropped.
- **Word features Markdown/SharePoint KB cannot store** won't appear in the UI: underline, fonts, font sizes, text color, alignment, line spacing, page layout, headers/footers, comments, track changes. Underline is the one Word users will look for. Consider a hint that suggests bold or highlight instead.

### Recommendation
- **Native and Word-like:** Option C′ (text flow + objects).
- **Simpler native fallback:** Option C (one box per paragraph). Users have to accept selecting whole blocks at a time, as in Notion.
- **Fully custom:** Option D, only if RichEditBox's limits block the work.
- **Not native:** Option B (WebView2) gets to WYSIWYG fastest. Keep it only as a fallback.

---

## 5. Rust-side libraries

| Need | Crate | Notes |
| --- | --- | --- |
| Markdown parse, validate, lint | `comrak` 1.2.0 | GFM tables, task lists, strikethrough, `highlight` (`==`), `math_dollars`, wikilinks, `front_matter_delimiter`. **`:::image` isn't supported** and needs a custom pass or preprocessor. |
| Front matter | `serde_yaml` (or `serde_yml`) | Read and write the Properties block. |
| Link checking | `comrak` AST + `walkdir` | Find `[[Page]]` targets that no longer exist. |
| WebView2 | `windows-webview` 0.100 (`reactor` feature) | Virtual host mapping, messaging, ExecuteScript. |
| File watching | `notify` | Reload when a file changes outside the app. |
| Upload (optional) | `reqwest` + `oauth2` / MSAL | Microsoft Graph (see below). |

Option B only: bundle the JS/CSS with Vite or esbuild into an `editor/` folder. Ship that folder next to the exe or embed it with `include_dir`.

---

## 6. Optional: publish straight to SharePoint

- The documentation says a knowledge base is a SharePoint library of `.md` files. In principle, uploading through **Microsoft Graph** (`PUT /sites/{site-id}/drives/{drive-id}/root:/{path}.md:/content`) should work.
- **Not verified**: none of the documentation describes a Graph or REST API for KB libraries. Confirm the following on a Frontier tenant before building this:
  1. whether the KB library appears as a drive
  2. whether uploaded files show up as KB pages
  3. whether subpage hierarchy follows folders
- Until then, the reliable route is to **save to a local folder → New > Import in SharePoint**.

---

## 7. Proposed app features (beyond the syntax toolbar)

- Workspace = a folder. A TreeView of pages (mirrors the KB hierarchy) and TabView for multiple open pages.
- Toolbar is a WinUI `CommandBar`. Primary commands are the formatting toggles. Insert commands sit in a `DropDownButton` or `MenuFlyout`: table, image, video, link, wiki link, code, math, Mermaid, divider.
- A **Visual | Source** switch (SelectorBar or segmented ToggleButtons), the same model as SharePoint.
- **Properties** pane (SplitView or Expander) for front matter key/value editing, with "Add property".
- Pre-export checks: local image paths (imports don't copy images), broken `[[links]]`, and duplicate page names.
- Rename page with automatic `[[link]]` updates. This addresses the rename limitation SharePoint documents.
- Light and dark theme, following WinUI. Pass the theme to the editor's CSS.

---

## 8. Risks and open questions

1. **Everything is in preview.** The SharePoint KB syntax and `windows-reactor` APIs can both change. Keep the syntax rules in one module.
2. `:::image` attributes beyond `type`, `source`, and `alt-text` (for example `lightbox` or `border`, as on Learn) aren't documented for KB. Parse unknown attributes and keep them unchanged.
3. Undocumented edge cases: wiki links with labels or headings (`[[Page#Section]]`, `[[Page|Text]]`), table cell alignment, nested lists in tables. Test each one against SharePoint itself.
4. WebView2 Runtime is preinstalled on Windows 11. On Windows 10, use the Evergreen bootstrapper.
5. Packaging: an unpackaged exe with `windows-reactor-setup` is the simplest option. Use MSIX if you want Store or Intune distribution.

---

## Sources

- [Create and edit a SharePoint Knowledge Base manually (preview)](https://support.microsoft.com/en-us/sharepoint/knowledge-base/create-edit-knowledge-base-manually)
- [Plan and set up SharePoint Knowledge Base (preview) — Microsoft Learn](https://learn.microsoft.com/en-us/sharepoint/sharepoint-knowledge-base)
- [Get started with SharePoint Knowledge Base (preview)](https://support.microsoft.com/sharepoint/knowledge-base/get-started-with-sharepoint-knowledge-base)
- [Introducing a team knowledge base — SharePoint blog](https://techcommunity.microsoft.com/blog/spblog/introducing-a-team-knowledge-base-curated-by-copilot-and-agents/4564203)
- [4sysops: SharePoint knowledge base](https://4sysops.com/archives/sharepoint-knowledge-base-copilot-curates-markdown-pages-for-ai-agents/)
- [HubSite365: Import Markdown files into a SharePoint KB](https://www.hubsite365.com/en-ww/crm-pages/import-markdown-files-into-a-sharepoint-knowledge-base.htm)
- [Rust for Windows — November 2026 (windows-rs #5445)](https://github.com/microsoft/windows-rs/issues/5445)
- [windows-reactor on docs.rs](https://docs.rs/windows-reactor/latest/windows_reactor/) · [crates.io](https://crates.io/crates/windows-reactor) · [readme](https://github.com/microsoft/windows-rs/blob/master/crates/libs/reactor/readme.md)
- [windows-webview on docs.rs](https://docs.rs/windows-webview/latest/windows_webview/)
- [winui3-rs (unofficial)](https://github.com/Alovchin91/winui3-rs)
- Native math: [RaTeX](https://github.com/erweixin/RaTeX) · [LaTeX-Rust](https://github.com/baselogic/LaTeX-Rust) · [rightkit_typeset](https://docs.rs/rightkit-typeset/latest/rightkit_typeset/)
- Native Mermaid: [mermaid-rs-renderer](https://github.com/1jehuang/mermaid-rs-renderer) · [pele-rs](https://github.com/nexmoe/pele-rs) · [mermaid-to-svg](https://github.com/warpdotdev/mermaid-to-svg) · [rusty-mermaid](https://github.com/base58ed/rusty-mermaid) · [fermaid](https://github.com/tenyi/fermaid)
- [comrak Extension options](https://docs.rs/comrak/latest/comrak/options/struct.Extension.html)
- [Rust Windows GUI guide 2026](https://rust-pc.github.io/rust-windows-gui.html)
