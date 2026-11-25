// The page: Tiptap (ProseMirror) for the visual view, a plain text box for the Markdown
// source, talking to the app through WebView2's message channel.
//
// The app owns files, Markdown, the ribbon and the panes. The page only ever sees ProseMirror
// JSON (src/bridge.rs has the node and mark names) or, in the source view, Markdown text.
//
// From the app
//   load {doc}                    a new document (undo starts empty)
//   source {markdown}             show the Markdown view with this text
//   visual                        show the visual view again, as it was
//   settings {zoom, fit, darkPage, bodyPx, cursorSpace, windowDark}
//   command {name, ...}           an editing command (see `commands`)
//   get {id, what}                reply {id, doc | markdown | blocks}
//   find {query, matchCase, wholeWords}
//   select {from, to, ratio}      select a range and scroll it `ratio` down the window
//   replace {changes: [[from, to, text]]}
//   saved · focus
// To the app
//   ready · changed · idle {outline | source} · state {...} · reply {...} · matches {...}
//   shortcut {name} · wheel {delta} · openUrl {href} · editMedia {pos, attributes} · log {text}

import {
  Editor, Extension, Mark, Node, mergeAttributes,
  StarterKit, Code, CodeBlock, Highlight, TaskItem, TaskList,
  Table, TableCell, TableHeader, TableRow, Placeholder,
  Plugin, PluginKey, NodeSelection, TextSelection, Decoration, DecorationSet,
} from './vendor/tiptap.js';

const host = window.chrome?.webview;
const post = (message) => (host ? host.postMessage(message) : console.log('to app', message));
const log = (text) => post({ type: 'log', text: String(text) });

window.addEventListener('error', (event) => log(`error: ${event.message} at ${event.filename}:${event.lineno}`));
window.addEventListener('unhandledrejection', (event) => log(`unhandled: ${event.reason}`));

const page = document.getElementById('page');
const visual = document.getElementById('visual');
const source = document.getElementById('source');
const tableTools = document.getElementById('table-tools');

/** Grows a text box to fit its text, so the page scrolls rather than the box. */
function fit(textarea) {
  textarea.style.height = 'auto';
  const frame = textarea.offsetHeight - textarea.clientHeight;
  textarea.style.height = `${textarea.scrollHeight + frame}px`;
}

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

/** Runs `change(attrs)` on the node a node view shows, as one edit. */
function setAttrs(editor, getPos, attrs) {
  const pos = getPos();
  if (typeof pos !== 'number') return;
  editor.commands.command(({ tr }) => {
    const node = tr.doc.nodeAt(pos);
    if (!node) return false;
    tr.setNodeMarkup(pos, undefined, { ...node.attrs, ...attrs });
    return true;
  });
}

function removeNode(editor, getPos) {
  const pos = getPos();
  if (typeof pos !== 'number') return;
  editor.commands.command(({ tr }) => {
    const node = tr.doc.nodeAt(pos);
    if (!node) return false;
    tr.delete(pos, pos + node.nodeSize);
    return true;
  });
  editor.commands.focus();
}

function element(tag, className, text) {
  const node = document.createElement(tag);
  if (className) node.className = className;
  if (text !== undefined) node.textContent = text;
  return node;
}

/** A card's header: a title, an optional note or control, and a remove button. */
function cardHeader(title, removeTip, onRemove, extra) {
  const header = element('div', 'card-header');
  header.contentEditable = 'false';
  header.append(element('span', 'title', title));
  if (extra) header.append(extra);
  const remove = element('button', 'remove', '✕');
  remove.title = removeTip;
  remove.addEventListener('mousedown', (event) => event.preventDefault());
  remove.addEventListener('click', onRemove);
  header.append(remove);
  return header;
}

/**
 * A block whose text is an attribute, edited in a text box on a card: display math and
 * Markdown the visual editor keeps as written.
 */
function sourceBlock({ name, attribute, title, removeTip }) {
  return Node.create({
    name,
    group: 'block',
    atom: true,
    selectable: true,
    draggable: false,
    addAttributes() {
      return { [attribute]: { default: '' } };
    },
    parseHTML() {
      return [{ tag: `div[data-node="${name}"]` }];
    },
    renderHTML({ HTMLAttributes }) {
      return ['div', mergeAttributes(HTMLAttributes, { 'data-node': name })];
    },
    addNodeView() {
      return ({ node, getPos, editor }) => {
        const dom = element('div', 'card');
        dom.contentEditable = 'false';
        const text = element('textarea');
        text.spellcheck = false;
        text.value = node.attrs[attribute];
        text.addEventListener('input', () => {
          fit(text);
          setAttrs(editor, getPos, { [attribute]: text.value });
        });
        dom.append(cardHeader(title, removeTip, () => removeNode(editor, getPos)), text);
        requestAnimationFrame(() => fit(text));
        return {
          dom,
          update(updated) {
            if (updated.type.name !== name) return false;
            if (text.value !== updated.attrs[attribute]) {
              text.value = updated.attrs[attribute];
              fit(text);
            }
            return true;
          },
          // The text box handles its own keys and clicks.
          stopEvent: (event) => event.target === text,
          ignoreMutation: () => true,
        };
      };
    },
  });
}

/** Inline math and inline Markdown kept as written: shown as text, edited by double-click. */
function sourceInline({ name, className, tip }) {
  return Node.create({
    name,
    group: 'inline',
    inline: true,
    atom: true,
    selectable: true,
    addAttributes() {
      return { [name === 'mathInline' ? 'latex' : 'markdown']: { default: '' } };
    },
    parseHTML() {
      return [{ tag: `span[data-node="${name}"]` }];
    },
    renderHTML({ HTMLAttributes }) {
      return ['span', mergeAttributes(HTMLAttributes, { 'data-node': name })];
    },
    addNodeView() {
      const attribute = name === 'mathInline' ? 'latex' : 'markdown';
      return ({ node, getPos, editor }) => {
        const dom = element('span', className, node.attrs[attribute] || ' ');
        dom.title = tip;
        dom.contentEditable = 'false';
        let input = null;
        const finish = (keep) => {
          if (!input) return;
          const value = input.value;
          input.remove();
          input = null;
          dom.hidden = false;
          if (keep && value !== node.attrs[attribute]) setAttrs(editor, getPos, { [attribute]: value });
          editor.commands.focus();
        };
        dom.addEventListener('dblclick', () => {
          if (input) return;
          input = element('input', 'inline-edit');
          input.value = node.attrs[attribute];
          input.size = Math.max(4, input.value.length + 1);
          input.addEventListener('input', () => { input.size = Math.max(4, input.value.length + 1); });
          input.addEventListener('keydown', (event) => {
            if (event.key === 'Enter') { event.preventDefault(); finish(true); }
            if (event.key === 'Escape') { event.preventDefault(); finish(false); }
          });
          input.addEventListener('blur', () => finish(true));
          dom.hidden = true;
          dom.after(input);
          input.focus();
          input.select();
        });
        return {
          dom,
          update(updated) {
            if (updated.type.name !== name) return false;
            node = updated;
            dom.textContent = updated.attrs[attribute] || ' ';
            return true;
          },
          stopEvent: (event) => input !== null && event.target === input,
          ignoreMutation: () => true,
          destroy: () => input?.remove(),
        };
      };
    },
  });
}

const VIDEO = /\.(mp4|webm|mov|m4v|avi|wmv)(\?|#|$)|youtube\.com\/|youtu\.be\/|vimeo\.com\/|microsoftstream\.com\//i;
const attributeOf = (attributes, key) => (attributes.find(([name]) => name === key) || [])[1] || '';

/** A `:::image …:::` picture or video, with its address and alt text edited in the app. */
const Media = Node.create({
  name: 'media',
  group: 'block',
  atom: true,
  selectable: true,
  addAttributes() {
    return { attributes: { default: [] } };
  },
  parseHTML() {
    return [{ tag: 'div[data-node="media"]' }];
  },
  renderHTML({ HTMLAttributes }) {
    return ['div', mergeAttributes(HTMLAttributes, { 'data-node': 'media' })];
  },
  addNodeView() {
    return ({ node, getPos, editor }) => {
      const dom = element('div', 'card');
      dom.contentEditable = 'false';
      const draw = (attributes) => {
        dom.replaceChildren();
        const address = attributeOf(attributes, 'source');
        const alt = attributeOf(attributes, 'alt-text');
        const video = VIDEO.test(address);
        const edit = element('button', 'edit', 'Edit…');
        edit.title = 'Change the address or alt text';
        edit.addEventListener('mousedown', (event) => event.preventDefault());
        edit.addEventListener('click', () => post({ type: 'editMedia', pos: getPos(), attributes }));
        dom.append(cardHeader(video ? 'Video' : 'Picture', 'Remove', () => removeNode(editor, getPos), edit));
        const preview = element('div', 'media-preview');
        if (video) {
          preview.append(element('span', 'caption', '▶ Video – plays in SharePoint'));
        } else if (/^https?:\/\//i.test(address)) {
          const image = element('img');
          image.src = address;
          image.alt = alt;
          image.addEventListener('error', () => {
            image.replaceWith(element('span', 'caption', 'The picture could not be shown from this address.'));
          });
          preview.append(image);
        } else {
          preview.append(element('span', 'caption', 'Use a web address readers can reach – SharePoint does not copy local pictures.'));
        }
        dom.append(preview, element('div', 'caption', address || '(no address)'));
        if (alt) dom.append(element('div', 'caption', `Alt text: ${alt}`));
      };
      draw(node.attrs.attributes);
      dom.addEventListener('dblclick', () => post({ type: 'editMedia', pos: getPos(), attributes: node.attrs.attributes }));
      return {
        dom,
        update(updated) {
          if (updated.type.name !== 'media') return false;
          node = updated;
          draw(updated.attrs.attributes);
          return true;
        },
        ignoreMutation: () => true,
      };
    };
  },
});

/** Code blocks on a card, with their language (a Mermaid diagram when it is `mermaid`). */
const KbCodeBlock = CodeBlock.extend({
  addNodeView() {
    return ({ node, getPos, editor }) => {
      const dom = element('div', 'card code-card');
      const language = element('input');
      language.placeholder = 'Language (optional)';
      language.spellcheck = false;
      language.value = node.attrs.language || '';
      const note = element('span', 'note');
      const header = cardHeader('', 'Remove code block', () => removeNode(editor, getPos), language);
      header.querySelector('.title').after(language);
      language.after(note);
      const title = header.querySelector('.title');
      const label = (value) => {
        const mermaid = value.trim().toLowerCase() === 'mermaid';
        title.textContent = mermaid ? 'Mermaid diagram' : 'Code block';
        note.textContent = mermaid ? 'Renders as a diagram in SharePoint' : '';
      };
      label(language.value);
      language.addEventListener('input', () => {
        label(language.value);
        setAttrs(editor, getPos, { language: language.value.trim() || null });
      });
      const pre = element('pre');
      const code = element('code');
      pre.append(code);
      dom.append(header, pre);
      return {
        dom,
        contentDOM: code,
        update(updated) {
          if (updated.type.name !== 'codeBlock') return false;
          const value = updated.attrs.language || '';
          if (document.activeElement !== language && language.value !== value) {
            language.value = value;
            label(value);
          }
          return true;
        },
        stopEvent: (event) => header.contains(event.target),
        ignoreMutation: (mutation) => header.contains(mutation.target),
      };
    };
  },
});

/** Tables keep their columns' Markdown alignment, which the page does not show. */
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

/** The app's Word-style keys, where they differ from Tiptap's. */
const WordKeys = Extension.create({
  name: 'wordKeys',
  addKeyboardShortcuts() {
    return {
      'Mod-Shift-x': () => this.editor.commands.toggleStrike(),
      'Mod-Shift-s': () => true, // Save As, not strikethrough (handled before the editor).
      'Mod-Alt-c': () => this.editor.commands.toggleCode(),
      'Mod-e': () => true, // Tiptap's code key; the app's is Ctrl+Alt+C.
      'Mod-Shift-l': () => this.editor.commands.toggleBulletList(),
      'Mod-Shift-n': () => run({ name: 'kind', kind: 'normal' }),
      'Mod-Alt-0': () => run({ name: 'kind', kind: 'normal' }),
      ...Object.fromEntries([1, 2, 3, 4, 5, 6].map((level) =>
        [`Mod-Alt-${level}`, () => run({ name: 'kind', kind: 'heading', level })])),
    };
  },
});

// ---------------------------------------------------------------- search results

const findKey = new PluginKey('find');
const FindHighlights = Extension.create({
  name: 'findHighlights',
  addProseMirrorPlugins() {
    return [new Plugin({
      key: findKey,
      state: {
        init: () => DecorationSet.empty,
        apply(tr, set) {
          const meta = tr.getMeta(findKey);
          if (meta) {
            return DecorationSet.create(tr.doc, meta.map(({ from, to, current }) =>
              Decoration.inline(from, to, { class: current ? 'find-current' : 'find-match' })));
          }
          return set.map(tr.mapping, tr.doc);
        },
      },
      props: { decorations: (state) => findKey.getState(state) },
    })];
  },
});

// ---------------------------------------------------------------- the editor

const extensions = [
  StarterKit.configure({
    code: false,
    codeBlock: false,
    underline: false,
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
  sourceInline({ name: 'mathInline', className: 'math-inline', tip: 'Equation (double-click to edit)' }),
  sourceInline({ name: 'rawInline', className: 'raw-inline', tip: 'Markdown kept as written (double-click to edit)' }),
  sourceBlock({ name: 'mathBlock', attribute: 'latex', title: 'Equation (LaTeX) – shown as $$ … $$', removeTip: 'Remove equation' }),
  sourceBlock({ name: 'rawBlock', attribute: 'markdown', title: 'Markdown the visual editor keeps as written', removeTip: 'Remove' }),
  Media,
  WordKeys,
  FindHighlights,
  Placeholder.configure({ placeholder: 'Start writing your page…' }),
];

let editor = null;
let mode = 'visual';
let dirty = false;
let settings = { zoom: 1, fit: false, darkPage: false, bodyPx: 14, cursorSpace: 33 };
/** Blank page added at the end after a heading jump, so the last heading can reach the top. */
let reachRoom = false;

function markDirty() {
  if (!dirty) {
    dirty = true;
    post({ type: 'changed' });
  }
}

/** A fresh editor for each document, so undo starts empty. */
function open(doc) {
  editor?.destroy();
  dirty = false;
  reachRoom = false;
  editor = new Editor({
    element: visual,
    extensions,
    content: doc,
    enableContentCheck: true,
    onContentError: ({ error }) => log(`content error: ${error.message}`),
    editorProps: {
      attributes: { spellcheck: 'true', lang: navigator.language || 'en' },
      handleClick(view, pos, event) {
        if (!event.ctrlKey) return false;
        const marks = view.state.doc.resolve(pos).marks();
        const link = marks.find((mark) => mark.type.name === 'link');
        if (link) {
          post({ type: 'openUrl', href: link.attrs.href });
          return true;
        }
        return false;
      },
    },
    onUpdate: () => {
      markDirty();
      scheduleIdle();
    },
    onSelectionUpdate: () => reportState(),
    onTransaction: ({ transaction }) => {
      if (transaction.docChanged) afterEdit();
      reportState();
      placeTableTools();
    },
  });
  applySettings();
  findMatches();
  sendIdle();
}

// ---------------------------------------------------------------- the app's view settings

function applySettings() {
  const root = document.documentElement;
  const zoom = settings.fit ? 1 : settings.zoom;
  root.style.setProperty('--zoom', String(zoom));
  root.style.setProperty('--body-size', `${settings.bodyPx / 16}rem`);
  root.classList.toggle('dark-page', !!settings.darkPage);
  // Scroll bars (and checkboxes) in the app's theme, not Windows'.
  root.style.colorScheme = settings.windowDark ? 'dark' : 'light';
  layoutPage();
  if (editor) {
    const below = Math.round(innerHeight * settings.cursorSpace / 100);
    editor.view.setProps({
      scrollThreshold: { top: 24, bottom: below, left: 0, right: 0 },
      scrollMargin: { top: 24, bottom: below, left: 0, right: 0 },
    });
  }
}

/** The page's width: as wide as the window leaves room for, between 320 and 900 pixels at
 * 100% and that times the zoom (it keeps its line breaks), or the whole room for Page Width. */
function layoutPage() {
  const room = document.documentElement.clientWidth - 48;
  const zoom = settings.fit ? 1 : settings.zoom;
  const base = settings.fit ? Math.max(room, 320) : Math.min(Math.max(room, 320), 900);
  page.style.width = `${base * zoom}px`;
  page.style.minHeight = `${base * zoom * 11 / 8.5}px`;
  const cursorRoom = innerHeight * settings.cursorSpace / 100;
  const end = reachRoom ? Math.max(cursorRoom, innerHeight) : cursorRoom;
  page.style.paddingBottom = `calc(3rem + ${end}px)`;
  // The Markdown's lines wrap at the new width.
  if (!source.hidden) fit(source);
  placeTableTools();
}
window.addEventListener('resize', applySettings);

// ---------------------------------------------------------------- telling the app

let stateQueued = false;
let lastState = '';
function reportState() {
  if (stateQueued) return;
  stateQueued = true;
  requestAnimationFrame(() => {
    stateQueued = false;
    const state = mode === 'source' ? sourceState() : visualState();
    const text = JSON.stringify(state);
    if (text !== lastState) {
      lastState = text;
      post({ type: 'state', ...state });
    }
  });
}

function visualState() {
  if (!editor) return {};
  const { state } = editor;
  const { from, to, empty } = state.selection;
  let kind = 'normal';
  let level = 0;
  for (let l = 1; l <= 6; l++) {
    if (editor.isActive('heading', { level: l })) { kind = 'heading'; level = l; }
  }
  if (kind === 'normal' && editor.isActive('blockquote')) kind = 'quote';
  let list = null;
  let checked = false;
  if (editor.isActive('taskItem')) {
    list = 'task';
    checked = !!editor.getAttributes('taskItem').checked;
  } else if (editor.isActive('orderedList')) list = 'numbered';
  else if (editor.isActive('bulletList')) list = 'bullet';
  const selected = state.doc.textBetween(from, to, '\n', ' ').slice(0, 500);
  return {
    bold: editor.isActive('bold') && kind !== 'heading',
    italic: editor.isActive('italic'),
    strike: editor.isActive('strike'),
    highlight: editor.isActive('highlight'),
    code: editor.isActive('code'),
    math: state.selection instanceof NodeSelection && state.selection.node.type.name === 'mathInline',
    kind,
    level,
    list,
    checked,
    hasSelection: !empty,
    selectedText: selected,
    caret: from,
  };
}

function sourceState() {
  const from = source.selectionStart;
  const to = source.selectionEnd;
  return { kind: 'normal', level: 0, list: null, hasSelection: to > from, selectedText: source.value.slice(from, to).slice(0, 500), caret: from };
}

/** The headings, with where each starts. */
function outline() {
  const headings = [];
  editor.state.doc.descendants((node, pos) => {
    if (node.type.name === 'heading') {
      headings.push({ level: node.attrs.level, text: node.textContent, pos });
      return false;
    }
    return node.type.name !== 'codeBlock';
  });
  return headings;
}

// Work that reads the whole document runs once typing pauses.
let idleTimer = 0;
function scheduleIdle() {
  clearTimeout(idleTimer);
  idleTimer = setTimeout(sendIdle, 250);
}
function sendIdle() {
  if (mode === 'source') post({ type: 'idle', source: source.value });
  else if (editor) post({ type: 'idle', outline: outline() });
  findMatches();
}

// ---------------------------------------------------------------- commands

function liftQuote(chain) {
  return editor.isActive('blockquote') ? chain.lift('blockquote') : chain;
}

const LIST_TYPES = { bullet: 'bulletList', numbered: 'orderedList', task: 'taskList' };
const MARKS = { bold: 'Bold', italic: 'Italic', strike: 'Strike', highlight: 'Highlight', code: 'Code' };

function insertLinked(mark, attrs, text) {
  const { state } = editor;
  const { from, to, empty } = state.selection;
  const selected = state.doc.textBetween(from, to, ' ', ' ');
  const label = text || (empty ? (attrs.href || attrs.page) : '');
  const chain = editor.chain().focus();
  if (label && label !== selected) {
    chain.insertContent({ type: 'text', text: label, marks: [{ type: mark, attrs }] }).run();
  } else {
    chain.setMark(mark, attrs).run();
  }
}

function toggleMath() {
  const { state } = editor;
  const { selection } = state;
  if (selection instanceof NodeSelection && selection.node.type.name === 'mathInline') {
    const latex = selection.node.attrs.latex;
    editor.chain().focus().command(({ tr }) => {
      tr.replaceWith(selection.from, selection.to, latex ? state.schema.text(latex) : []);
      return true;
    }).run();
    return;
  }
  const latex = state.doc.textBetween(selection.from, selection.to, ' ', ' ') || 'x';
  editor.chain().focus().insertContent({ type: 'mathInline', attrs: { latex } }).run();
}

function sourceCommand(name) {
  source.focus();
  if (name === 'undo' || name === 'redo') document.execCommand(name);
}

function run(command) {
  if (mode === 'source') {
    sourceCommand(command.name);
    return true;
  }
  if (!editor) return false;
  const chain = () => editor.chain().focus();
  switch (command.name) {
    case 'undo': chain().undo().run(); break;
    case 'redo': chain().redo().run(); break;
    case 'inline':
      if (command.mark === 'math') toggleMath();
      else chain()[`toggle${MARKS[command.mark]}`]().run();
      break;
    case 'kind':
      if (command.kind === 'quote') {
        if (editor.isActive('blockquote')) chain().lift('blockquote').run();
        else chain().setParagraph().wrapIn('blockquote').run();
      } else if (command.kind === 'heading') {
        liftQuote(chain()).setHeading({ level: command.level }).run();
      } else {
        liftQuote(chain()).setParagraph().run();
      }
      break;
    case 'list': {
      const type = LIST_TYPES[command.list];
      const toggle = { bulletList: 'toggleBulletList', orderedList: 'toggleOrderedList', taskList: 'toggleTaskList' }[type];
      chain()[toggle]().run();
      break;
    }
    case 'toggleDone':
      if (editor.isActive('taskItem')) {
        chain().updateAttributes('taskItem', { checked: !editor.getAttributes('taskItem').checked }).run();
      } else {
        chain().toggleTaskList().run();
      }
      break;
    case 'indent':
      if (!chain().sinkListItem('listItem').run()) chain().sinkListItem('taskItem').run();
      break;
    case 'outdent':
      if (!chain().liftListItem('listItem').run()) chain().liftListItem('taskItem').run();
      break;
    case 'link': insertLinked('link', { href: command.href }, command.text); break;
    case 'pageLink': insertLinked('pageLink', { page: command.page }, command.text); break;
    case 'insert': chain().insertContent(command.node).run(); break;
    case 'setAttrs':
      editor.chain().focus().command(({ tr }) => {
        const node = tr.doc.nodeAt(command.pos);
        if (!node) return false;
        tr.setNodeMarkup(command.pos, undefined, { ...node.attrs, ...command.attrs });
        return true;
      }).run();
      break;
    case 'focus': editor.commands.focus(); break;
    default: log(`unknown command ${command.name}`); return false;
  }
  return true;
}

// The table's own commands, on a bar above it while the cursor is in it.
function placeTableTools() {
  if (!editor || mode !== 'visual') { tableTools.hidden = true; return; }
  const { $from } = editor.state.selection;
  let tablePos = null;
  for (let depth = $from.depth; depth > 0; depth--) {
    if ($from.node(depth).type.name === 'table') { tablePos = $from.before(depth); break; }
  }
  if (tablePos === null) { tableTools.hidden = true; return; }
  let dom = editor.view.nodeDOM(tablePos);
  if (dom && dom.nodeType !== 1) dom = dom.parentElement;
  if (!dom) { tableTools.hidden = true; return; }
  const pageBox = page.getBoundingClientRect();
  const box = dom.getBoundingClientRect();
  tableTools.hidden = false;
  tableTools.style.left = `${box.left - pageBox.left}px`;
  tableTools.style.top = `${box.top - pageBox.top - tableTools.offsetHeight - 4}px`;
}
tableTools.addEventListener('mousedown', (event) => event.preventDefault());
tableTools.addEventListener('click', (event) => {
  const name = event.target.closest('button')?.dataset.table;
  if (name && editor) editor.chain().focus()[name]().run();
});

// ---------------------------------------------------------------- find and replace

let search = { query: '', matchCase: false, wholeWords: false };
let matches = [];
let currentMatch = null;
const WORD = /[\p{L}\p{N}_]/u;
const escape = (text) => text.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');

/** Every match in `text`, as [start, end] offsets. */
function findIn(text) {
  const found = [];
  if (!search.query) return found;
  const pattern = new RegExp(escape(search.query), search.matchCase ? 'gu' : 'giu');
  for (const match of text.matchAll(pattern)) {
    const start = match.index;
    const end = start + match[0].length;
    if (end === start) continue;
    if (search.wholeWords && (WORD.test(text[start - 1] || '') || WORD.test(text[end] || ''))) continue;
    found.push([start, end]);
  }
  return found;
}

const snippet = (text, start, end) => ({
  before: text.slice(Math.max(0, start - 30), start).replace(/￼/g, ' ').trimStart(),
  text: text.slice(start, end),
  after: text.slice(end, end + 40).replace(/￼/g, ' ').trimEnd(),
});

function findMatches() {
  matches = [];
  if (search.query) {
    if (mode === 'source') {
      for (const [start, end] of findIn(source.value)) {
        matches.push({ from: start, to: end, ...snippet(source.value, start, end) });
      }
    } else if (editor) {
      // Within a textblock, each character and each inline node is one position, so the
      // text with a placeholder for inline nodes lines up with the document.
      editor.state.doc.descendants((node, pos) => {
        if (!node.isTextblock) return true;
        const text = node.textBetween(0, node.content.size, undefined, '￼');
        for (const [start, end] of findIn(text)) {
          matches.push({ from: pos + 1 + start, to: pos + 1 + end, ...snippet(text, start, end) });
        }
        return false;
      });
    }
  }
  if (currentMatch && !matches.some((m) => m.from === currentMatch.from && m.to === currentMatch.to)) currentMatch = null;
  highlightMatches();
  post({ type: 'matches', query: search.query, matches });
}

function highlightMatches() {
  if (!editor || mode !== 'visual') return;
  const meta = matches.map(({ from, to }) => ({
    from, to, current: currentMatch && currentMatch.from === from && currentMatch.to === to,
  }));
  editor.view.dispatch(editor.state.tr.setMeta(findKey, meta).setMeta('addToHistory', false));
}

function replaceRanges(changes) {
  const sorted = [...changes].sort((a, b) => b[0] - a[0]);
  if (mode === 'source') {
    for (const [from, to, text] of sorted) source.setRangeText(text, from, to, 'end');
    fit(source);
    markDirty();
    sendIdle();
    return;
  }
  if (!editor || !sorted.length) return;
  editor.chain().command(({ tr }) => {
    for (const [from, to, text] of sorted) {
      if (text) tr.insertText(text, from, to);
      else tr.delete(from, to);
    }
    return true;
  }).run();
  sendIdle();
}

// ---------------------------------------------------------------- selecting and scrolling

/** Selects from–to and scrolls it `ratio` of the way down the window (0: the top, with a
 * little room above, as Word jumps to a heading). */
function select(from, to, ratio) {
  let top;
  if (mode === 'source') {
    source.focus({ preventScroll: true });
    source.setSelectionRange(from, to);
    top = source.getBoundingClientRect().top + offsetInSource(from);
  } else {
    if (!editor) return;
    const size = editor.state.doc.content.size;
    from = Math.max(0, Math.min(from, size));
    to = Math.max(from, Math.min(to, size));
    editor.chain().setTextSelection({ from, to }).run();
    editor.view.focus();
    currentMatch = matches.find((m) => m.from === from && m.to === to) || null;
    highlightMatches();
    top = editor.view.coordsAtPos(from).top;
  }
  if (ratio === 0 && !reachRoom) {
    reachRoom = true;
    layoutPage();
  }
  const margin = ratio === 0 ? 16 : 0;
  window.scrollTo({ top: Math.max(0, scrollY + top - ratio * innerHeight - margin) });
}

// ---------------------------------------------------------------- the Markdown view

/** How far below the top of the Markdown text box the text at `offset` starts. Lines wrap,
 * so this lays the text out in a hidden copy of the box, with the same width and type, and
 * measures where a marker at the offset lands. */
function offsetInSource(offset) {
  const style = getComputedStyle(source);
  const mirror = document.createElement('div');
  for (const name of ['boxSizing', 'width', 'paddingTop', 'paddingRight', 'paddingBottom',
    'paddingLeft', 'borderTopWidth', 'borderRightWidth', 'borderBottomWidth', 'borderLeftWidth',
    'fontFamily', 'fontSize', 'fontWeight', 'lineHeight', 'letterSpacing', 'tabSize',
    'whiteSpace', 'overflowWrap', 'wordBreak']) {
    mirror.style[name] = style[name];
  }
  Object.assign(mirror.style, { position: 'absolute', visibility: 'hidden', top: '0', left: '-9999px', borderStyle: 'solid' });
  mirror.textContent = source.value.slice(0, offset);
  const marker = document.createElement('span');
  marker.textContent = '​';
  mirror.append(marker);
  document.body.append(mirror);
  const top = marker.offsetTop;
  mirror.remove();
  return top;
}

function showSource(markdown) {
  mode = 'source';
  visual.hidden = true;
  tableTools.hidden = true;
  source.hidden = false;
  source.value = markdown;
  fit(source);
  source.focus({ preventScroll: true });
  source.setSelectionRange(0, 0);
  window.scrollTo(0, 0);
  reportState();
  sendIdle();
}

function showVisual() {
  mode = 'visual';
  source.hidden = true;
  visual.hidden = false;
  editor?.commands.focus();
  reportState();
  sendIdle();
}

source.addEventListener('input', () => {
  fit(source);
  markDirty();
  scheduleIdle();
});
source.addEventListener('select', reportState);
source.addEventListener('keyup', reportState);
source.addEventListener('mouseup', reportState);
source.addEventListener('keydown', (event) => {
  // Tab indents in the Markdown, as in a code editor.
  if (event.key === 'Tab' && !event.ctrlKey && !event.altKey) {
    event.preventDefault();
    document.execCommand('insertText', false, '    ');
  }
});

// ---------------------------------------------------------------- spelling

/** The prose in the page, block by block, as runs of text at their positions. Code is left
 * out: it is not prose. */
function textBlocks() {
  const blocks = [];
  editor.state.doc.descendants((node, pos) => {
    if (node.type.name === 'codeBlock') return false;
    if (!node.isTextblock) return true;
    const runs = [];
    node.forEach((child, offset) => {
      if (child.isText && !child.marks.some((mark) => mark.type.name === 'code')) {
        runs.push({ pos: pos + 1 + offset, text: child.text });
      }
    });
    if (runs.length) blocks.push(runs);
    return false;
  });
  return blocks;
}

// ---------------------------------------------------------------- keys the app handles

/** Keys for the app (files, search, zoom, the link dialog): WebView2 keeps keys to itself,
 * so the page passes them on. */
function appShortcut(event) {
  const key = event.key.toLowerCase();
  const ctrl = event.ctrlKey;
  const shift = event.shiftKey;
  if (event.altKey) return null;
  if (event.key === 'F3') return shift ? 'findPrevious' : 'findNext';
  if (event.key === 'F12') return ctrl && !shift ? 'open' : shift && !ctrl ? 'save' : !ctrl ? 'saveAs' : null;
  if (!ctrl) return null;
  if (key === 's') return shift ? 'saveAs' : 'save';
  if (shift) return null;
  return {
    o: 'open', n: 'new', f: 'find', h: 'replace', k: 'link',
    '=': 'zoomIn', '+': 'zoomIn', '-': 'zoomOut', '0': 'zoomReset',
  }[key] || (event.code === 'NumpadAdd' ? 'zoomIn' : event.code === 'NumpadSubtract' ? 'zoomOut' : null);
}

window.addEventListener('keydown', (event) => {
  const name = appShortcut(event);
  if (name) {
    event.preventDefault();
    event.stopPropagation();
    post({ type: 'shortcut', name });
  }
}, true);

// Ctrl+wheel zooms the page, in the app's steps.
window.addEventListener('wheel', (event) => {
  if (!event.ctrlKey) return;
  event.preventDefault();
  post({ type: 'wheel', delta: event.wheelDelta || -event.deltaY });
}, { passive: false });

// Dropping a file would open it in the page; the app opens files.
window.addEventListener('dragover', (event) => {
  if (event.dataTransfer?.types.includes('Files')) event.preventDefault();
});
window.addEventListener('drop', (event) => {
  if (event.dataTransfer?.types.includes('Files')) event.preventDefault();
});

// ---------------------------------------------------------------- measuring typing

const afterPaint = () => new Promise((resolve) => requestAnimationFrame(() => setTimeout(resolve, 0)));
let keyAt = null;
const typing = [];
window.addEventListener('keydown', () => { if (keyAt === null) keyAt = performance.now(); }, true);
window.addEventListener('keyup', () => setTimeout(() => { keyAt = null; }, 50), true);
function afterEdit() {
  if (keyAt === null) return;
  const started = keyAt;
  keyAt = null;
  afterPaint().then(() => {
    typing.push(performance.now() - started);
    if (typing.length >= 200) {
      const sorted = typing.splice(0).sort((a, b) => a - b);
      log(`typing: median ${sorted[100].toFixed(1)} ms, 95th ${sorted[190].toFixed(1)} ms, max ${sorted[199].toFixed(1)} ms`);
    }
  });
}

// ---------------------------------------------------------------- the app's messages

host?.addEventListener('message', (event) => {
  const message = event.data;
  switch (message.type) {
    case 'load':
      if (mode === 'source') showVisual();
      open(message.doc);
      if (message.focus !== false) editor.commands.focus('start');
      window.scrollTo(0, 0);
      break;
    case 'source': showSource(message.markdown); break;
    case 'visual': showVisual(); break;
    case 'settings':
      settings = { ...settings, ...message };
      applySettings();
      break;
    case 'command': run(message); break;
    case 'get': {
      const reply = { type: 'reply', id: message.id };
      if (message.what === 'markdown') reply.markdown = source.value;
      else if (message.what === 'blocks') reply.blocks = editor ? textBlocks() : [];
      else reply.doc = editor ? editor.getJSON() : null;
      post(reply);
      break;
    }
    case 'find':
      search = { query: message.query || '', matchCase: !!message.matchCase, wholeWords: !!message.wholeWords };
      findMatches();
      break;
    case 'select': select(message.from, message.to, message.ratio ?? 0.33); break;
    case 'replace': replaceRanges(message.changes || []); break;
    case 'saved': dirty = false; break;
    case 'focus':
      if (mode === 'source') source.focus();
      else editor?.commands.focus();
      break;
    default: log(`unknown message ${message.type}`);
  }
});

open({ type: 'doc', content: [{ type: 'paragraph' }] });
post({ type: 'ready' });
