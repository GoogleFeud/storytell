import * as monaco from "monaco-editor/esm/vs/editor/editor.api";
import { createSignal } from "solid-js";

const [editor, setEditorState] = createSignal<monaco.editor.IStandaloneCodeEditor>();

export const setEditor = (editor: monaco.editor.IStandaloneCodeEditor) => {
    setEditorState(editor);
};

export const setEditorText = (text: string) => {
    editor()?.setValue(text);
};