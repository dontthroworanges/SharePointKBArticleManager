// The article editor: Tiptap on the page, talking to the native app through WebView2's
// message channel. The app owns files and Markdown; this page only ever sees and returns
// ProseMirror JSON (see src/bridge.rs for the node and mark names).
//
// From the app:  load {doc} · command {name} · get {id} · saved · bench
// To the app:    ready · changed · state {text} · typing {text} · doc {id, doc} ·
//                shortcut {name} · bench {text} · log {text}

import {
  Editor, Extension, Mark, Node, mergeAttributes,
  StarterKit, Code, CodeBlock, Highlight, TaskItem, TaskList,
  Table, TableCell, TableHeader, TableRow, Placeholder,
} from './vendor/tiptap.js';

const host = window.chrome?.webview;
const post = (message) => host ? host.postMessage(message) : console.log('to app', message);
const log = (text) => post({ type: 'log', text: String(text) });

window.addEventListener('error', (event) => log(`error: ${event.message} at ${event.filename}:${event.lineno}`));
window.addEventListener('unhandledrejection', (event) => log(`unhandled: ${event.reason}`));

// ---------------------------------------------------------------- the dialect's extras

/** A `[[Page name]]` link to another Knowledge Base page. */
const PageLink = Mark.create({
  name: 'pageLink',
  inclusive: false,
  excludes: 'link',
  addAttributes() {
    return { page: { default: '' } };
  },
  parseHTML() {
    return [{ tag: 'a[data-page]', getAttrs: (dom) => ({ page: dom.getAttribute('data-page') }) }];
  },
  renderHTML({ mark }) {
    return ['a', { class: 'page-link', 'data-page': mark.attrs.page, title: `Page: ${mark.attrs.page}` }, 0];
  },
});

/**
 * A node the editor shows but does not edit as text (math, media, Markdown it has no editor
 * for). Its source is an attribute; double-clicking it asks for new source.
 */
function sourceAtom({ name, inline, attribute, label, className, render }) {
  return Node.create({
    name,
    group: inline ? 'inline' : 'block',
    inline,
    atom: true,
    selectable: true,
    draggable: !inline,
    addAttributes() {
      return { [attribute]: { default: attribute === 'attributes' ? [] : '' } };
    },
    parseHTML() {
      return [{ tag: `${inline ? 'span' : 'div'}[data-node="${name}"]` }];
    },
    renderHTML({ HTMLAttributes }) {
      return [inline ? 'span' : 'div', mergeAttributes(HTMLAttributes, { 'data-node': name })];
    },
    addNodeView() {
      return ({ node, getPos, editor }) => {
        const dom = document.createElement(inline ? 'span' : 'div');
        dom.className = `atom ${className}`;
        dom.contentEditable = 'false';
        const draw = (current) => {
          dom.replaceChildren();
          if (label && !inline) {
            const caption = document.createElement('span');
            caption.className = 'atom-label';
            caption.textContent = label;
            dom.append(caption);
          }
          render(dom, current.attrs[attribute]);
        };
        draw(node);
        dom.addEventListener('dblclick', () => {
          if (typeof render.edit !== 'function') return;
          const next = render.edit(node.attrs[attribute]);
          if (next === null || next === undefined) return;
          const pos = getPos();
          if (typeof pos !== 'number') return;
          editor.commands.command(({ tr }) => {
            tr.setNodeMarkup(pos, undefined, { ...node.attrs, [attribute]: next });
            return true;
          });
        });
        return {
          dom,
          update(updated) {
            if (updated.type.name !== name) return false;
            node = updated;
            draw(updated);
            return true;
          },
        };
      };
    },
  });
}

const promptFor = (title) => (value) => window.prompt(title, value);

const showText = (dom, text) => dom.append(document.createTextNode(text));
const mathInline = Object.assign((dom, latex) => showText(dom, latex), { edit: promptFor('Equation (LaTeX)') });
const mathBlock = Object.assign((dom, latex) => showText(dom, latex), { edit: promptFor('Equation (LaTeX)') });
const rawText = Object.assign((dom, markdown) => showText(dom, markdown), { edit: promptFor('Markdown') });
const media = Object.assign((dom, attributes) => {
  const get = (key) => (attributes.find(([name]) => name === key) || [])[1] || '';
  const source = get('source');
  const alt = get('alt-text');
  const video = /\.(mp4|webm|mov|m4v|avi|wmv)(\?|#|$)|youtube\.com\/|youtu\.be\/|vimeo\.com\/|microsoftstream\.com\//i.test(source);
  if (source && !video) {
    const image = document.createElement('img');
    image.src = source;
    image.alt = alt;
    dom.append(image);
  }
  const caption = document.createElement('span');
  caption.className = 'caption';
  caption.textContent = video ? `Video: ${source}` : alt || source;
  dom.append(caption);
}, {
  edit(attributes) {
    const source = window.prompt('Image or video address', (attributes.find(([k]) => k === 'source') || [])[1] || '');
    if (source === null) return null;
    const next = attributes.some(([k]) => k === 'source')
      ? attributes.map(([k, v]) => [k, k === 'source' ? source : v])
      : [...attributes, ['source', source]];
    return next;
  },
});

const MathInline = sourceAtom({ name: 'mathInline', inline: true, attribute: 'latex', className: 'math-inline', render: mathInline });
const MathBlock = sourceAtom({ name: 'mathBlock', inline: false, attribute: 'latex', label: 'Equation', className: 'math-block', render: mathBlock });
const RawInline = sourceAtom({ name: 'rawInline', inline: true, attribute: 'markdown', className: 'raw-inline', render: rawText });
const RawBlock = sourceAtom({ name: 'rawBlock', inline: false, attribute: 'markdown', label: 'Markdown', className: 'raw-block', render: rawText });
const Media = sourceAtom({ name: 'media', inline: false, attribute: 'attributes', className: 'media', render: media });

/** Tables keep their columns' Markdown alignment, which the editor does not show yet. */
const KbTable = Table.extend({
  addAttributes() {
    return {
      ...this.parent?.(),
      alignments: { default: [], parseHTML: () => [], renderHTML: () => ({}) },
    };
  },
});

/** Markdown allows code with other formatting (**`x`**); Tiptap's code mark excludes all. */
const KbCode = Code.extend({ excludes: '' });

/** The language shows in the block's corner. */
const KbCodeBlock = CodeBlock.extend({
  renderHTML({ node, HTMLAttributes }) {
    return ['pre', mergeAttributes(HTMLAttributes, { 'data-language': node.attrs.language || '' }),
      ['code', { class: node.attrs.language ? `language-${node.attrs.language}` : null }, 0]];
  },
});

/** The app's Word-style keys, where they differ from Tiptap's. */
const WordKeys = Extension.create({
  name: 'wordKeys',
  addKeyboardShortcuts() {
    return {
      'Mod-Shift-x': () => this.editor.commands.toggleStrike(),
      'Mod-Alt-c': () => this.editor.commands.toggleCode(),
      'Mod-Shift-l': () => this.editor.commands.toggleBulletList(),
      'Mod-Shift-n': () => this.editor.commands.setParagraph(),
      'Mod-k': () => { run('link'); return true; },
    };
  },
});

// ---------------------------------------------------------------- the editor

const extensions = [
  StarterKit.configure({
    code: false,
    codeBlock: false,
    underline: false,
    // Strike's own Ctrl+Shift+S is Save As here; WordKeys gives it Ctrl+Shift+X.
    strike: { HTMLAttributes: {} },
    link: { openOnClick: false, autolink: true, linkOnPaste: true, HTMLAttributes: { rel: null, target: null } },
    heading: { levels: [1, 2, 3, 4, 5, 6] },
  }),
  KbCode,
  KbCodeBlock,
  Highlight,
  TaskList,
  TaskItem.configure({ nested: true }),
  KbTable.configure({ resizable: false }),
  TableRow,
  TableHeader,
  TableCell,
  PageLink,
  MathInline,
  MathBlock,
  RawInline,
  RawBlock,
  Media,
  WordKeys,
  Placeholder.configure({ placeholder: 'Start writing your page…' }),
];

const page = document.getElementById('page');
let editor = null;
let dirty = false;
let lastState = '';

/** A fresh editor for each document, so undo starts empty. */
function open(doc) {
  editor?.destroy();
  dirty = false;
  editor = new Editor({
    element: page,
    extensions,
    content: doc,
    enableContentCheck: true,
    onContentError: ({ error }) => log(`content error: ${error.message}`),
    editorProps: {
      attributes: { spellcheck: 'true', lang: 'en' },
      // Typing keeps room below the line, as the app's "Space below the cursor" does.
      scrollThreshold: { top: 40, bottom: 160, left: 0, right: 0 },
      scrollMargin: { top: 40, bottom: 160, left: 0, right: 0 },
    },
    onUpdate: () => {
      if (!dirty) {
        dirty = true;
        post({ type: 'changed' });
      }
    },
    onSelectionUpdate: reportState,
    onTransaction: ({ transaction }) => {
      if (transaction.docChanged) afterEdit();
      reportState();
    },
  });
  editor.commands.focus('start');
  reportState();
}

let stateQueued = false;
function reportState() {
  if (stateQueued) return;
  stateQueued = true;
  requestAnimationFrame(() => {
    stateQueued = false;
    if (!editor) return;
    let block = 'Normal';
    for (let level = 1; level <= 6; level++) {
      if (editor.isActive('heading', { level })) block = `Heading ${level}`;
    }
    if (editor.isActive('codeBlock')) block = 'Code block';
    if (editor.isActive('blockquote')) block += ' · Quote';
    if (editor.isActive('taskList')) block += ' · Task list';
    else if (editor.isActive('orderedList')) block += ' · Numbered list';
    else if (editor.isActive('bulletList')) block += ' · Bulleted list';
    if (editor.isActive('table')) block += ' · Table';
    const marks = [['bold', 'Bold'], ['italic', 'Italic'], ['strike', 'Strikethrough'],
      ['highlight', 'Highlight'], ['code', 'Code'], ['link', 'Link'], ['pageLink', 'Page link']]
      .filter(([name]) => editor.isActive(name)).map(([, label]) => label);
    const words = editor.state.doc.textContent.split(/\s+/).filter(Boolean).length;
    const text = [block, ...marks, `${words} words`].join(' · ');
    if (text !== lastState) {
      lastState = text;
      post({ type: 'state', text });
    }
  });
}

const commands = {
  paragraph: (c) => c.setParagraph(),
  heading1: (c) => c.toggleHeading({ level: 1 }),
  heading2: (c) => c.toggleHeading({ level: 2 }),
  heading3: (c) => c.toggleHeading({ level: 3 }),
  bold: (c) => c.toggleBold(),
  italic: (c) => c.toggleItalic(),
  strike: (c) => c.toggleStrike(),
  highlight: (c) => c.toggleHighlight(),
  code: (c) => c.toggleCode(),
  bulletList: (c) => c.toggleBulletList(),
  orderedList: (c) => c.toggleOrderedList(),
  taskList: (c) => c.toggleTaskList(),
  blockquote: (c) => c.toggleBlockquote(),
  codeBlock: (c) => c.toggleCodeBlock(),
  rule: (c) => c.setHorizontalRule(),
  table: (c) => c.insertTable({ rows: 3, cols: 2, withHeaderRow: true }),
  undo: (c) => c.undo(),
  redo: (c) => c.redo(),
};

function run(name) {
  if (!editor) return;
  if (name === 'link') {
    const current = editor.getAttributes('link').href || '';
    const href = window.prompt('Link address (empty to remove)', current);
    if (href === null) return;
    const chain = editor.chain().focus().extendMarkRange('link');
    (href ? chain.setLink({ href }) : chain.unsetLink()).run();
    return;
  }
  const command = commands[name];
  if (command) command(editor.chain().focus()).run();
  else log(`unknown command ${name}`);
}

// File shortcuts go to the app; WebView2 keeps keys to itself, so the app never sees them.
window.addEventListener('keydown', (event) => {
  if (!event.ctrlKey || event.altKey) return;
  const key = event.key.toLowerCase();
  const name = key === 's' ? (event.shiftKey ? 'saveAs' : 'save')
    : key === 'o' && !event.shiftKey ? 'open'
    : key === 'n' && !event.shiftKey ? 'new'
    : null;
  if (name) {
    event.preventDefault();
    event.stopPropagation();
    post({ type: 'shortcut', name });
  }
}, true);

// ---------------------------------------------------------------- measuring typing

/** Resolves after the next frame has been drawn. */
const afterPaint = () => new Promise((resolve) => requestAnimationFrame(() => setTimeout(resolve, 0)));

// Key to painted frame, for real typing.
const samples = [];
let keyAt = null;
window.addEventListener('keydown', () => { if (keyAt === null) keyAt = performance.now(); }, true);
document.addEventListener('beforeinput', () => { if (keyAt === null) keyAt = performance.now(); }, true);
let typingQueued = false;
function afterEdit() {
  if (keyAt === null) return;
  const started = keyAt;
  keyAt = null;
  afterPaint().then(() => {
    samples.push(performance.now() - started);
    if (samples.length > 500) samples.shift();
    if (!typingQueued) {
      typingQueued = true;
      setTimeout(() => { typingQueued = false; post({ type: 'typing', text: summary('Typing', samples) }); }, 500);
    }
  });
}
// Keys that change nothing (arrows) should not leave a stale start time.
window.addEventListener('keyup', () => setTimeout(() => { keyAt = null; }, 50), true);

function summary(label, values) {
  if (!values.length) return '';
  const sorted = [...values].sort((a, b) => a - b);
  const at = (q) => sorted[Math.min(sorted.length - 1, Math.floor(q * sorted.length))];
  return `${label}: median ${at(0.5).toFixed(1)} ms, 95th ${at(0.95).toFixed(1)} ms, max ${sorted[sorted.length - 1].toFixed(1)} ms (${values.length} keys)`;
}

/**
 * Types 300 characters at the end of the document, one per frame, timing each to the frame
 * that shows it, and watching for the page scrolling up (the jump the RichEdit version had).
 * The document is put back afterwards.
 */
async function bench() {
  if (!editor) return;
  const before = editor.getJSON();
  const wasDirty = dirty;
  const scroller = document.scrollingElement;
  editor.commands.focus('end');
  let ups = 0;
  let last = scroller.scrollTop;
  const onScroll = () => {
    if (scroller.scrollTop < last - 1) ups++;
    last = scroller.scrollTop;
  };
  window.addEventListener('scroll', onScroll);
  const text = 'The quick brown fox jumps over the lazy dog. ';
  const times = [];
  for (let i = 0; i < 300; i++) {
    const started = performance.now();
    if (i % 75 === 74) {
      editor.commands.splitBlock();
      editor.view.dispatch(editor.state.tr.scrollIntoView());
    } else {
      editor.view.dispatch(editor.state.tr.insertText(text[i % text.length]).scrollIntoView());
    }
    await afterPaint();
    times.push(performance.now() - started);
  }
  window.removeEventListener('scroll', onScroll);
  const words = before.content?.length ?? 0;
  open(before);
  dirty = wasDirty;
  editor.commands.focus('end');
  post({ type: 'bench', text: `${summary('Benchmark', times).replace('keys', 'characters')}; page scrolled up ${ups} times; ${words} top-level blocks` });
}

// ---------------------------------------------------------------- the app's messages

host?.addEventListener('message', (event) => {
  const message = event.data;
  switch (message.type) {
    case 'load': open(message.doc); break;
    case 'command': run(message.name); break;
    case 'get': post({ type: 'doc', id: message.id, doc: editor.getJSON() }); break;
    case 'saved': dirty = false; break;
    case 'bench': bench(); break;
    default: log(`unknown message ${message.type}`);
  }
});

open({ type: 'doc', content: [{ type: 'paragraph' }] });
post({ type: 'ready' });
