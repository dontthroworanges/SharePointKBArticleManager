// Everything editor.js uses from Tiptap, bundled into vendor/tiptap.js by `npm run build`
// (one copy of ProseMirror for all extensions).
export { Editor, Extension, Mark, Node, mergeAttributes } from '@tiptap/core';
export { StarterKit } from '@tiptap/starter-kit';
export { Code } from '@tiptap/extension-code';
export { CodeBlock } from '@tiptap/extension-code-block';
export { Highlight } from '@tiptap/extension-highlight';
export { TaskItem, TaskList } from '@tiptap/extension-list';
export { Table, TableCell, TableHeader, TableRow } from '@tiptap/extension-table';
export { Placeholder } from '@tiptap/extensions';
export { Plugin, PluginKey, NodeSelection, TextSelection } from '@tiptap/pm/state';
export { Decoration, DecorationSet } from '@tiptap/pm/view';
