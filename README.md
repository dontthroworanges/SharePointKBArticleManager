# SharePoint Article Manager

> [!IMPORTANT]
> **Disclaimer:** This application is being built with AI-generated code. If you prefer not to use AI-generated software, please do not proceed.

The point of this application is to offer a simple, desktop based, Word-Like experience for creating and editing Markdown files specifically for the upcomming SharePoint Knowledge Base document library. 

## Current Features
- Built in Rust with Windows native WebUI3. 
- Document canvas uses WebView2 for best speed and compatibility with SharePoint format. 
- Visual and Markdown editors with standard text formatting
- Document Navigation based on Heading tags
- Find and Replace
- Windows native spell check

## Planned Features
These planned features have yet to be explorered so they may not be feasible to implement. 
[] Local Document management and quick linking
[] Direct connection to SharePoint Document library
[] Microsoft Word .DOCX Import and conversion
[] Export of Markdown into HTML or PDF. 

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
