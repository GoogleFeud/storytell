import * as monaco from "monaco-editor/esm/vs/editor/editor.api";
import { createSignal } from "solid-js";
import { setState, state } from ".";
import { recompileFile } from "./file";

const [editor, setEditorState] = createSignal<monaco.editor.IStandaloneCodeEditor>();

export const setEditor = (editor: monaco.editor.IStandaloneCodeEditor) => {
    setEditorState(editor);
    let prevFile: number|undefined;
    editor.onDidChangeModelContent(() => {
        if (state.currentFile) {
            if (prevFile && state.currentFile !== prevFile) {
                prevFile = state.currentFile;
                return;
            } else prevFile = state.currentFile;
            const content = editor.getValue();
            const currentContent = state.contents[state.currentFile];
            if (currentContent.textContent === content) return;
            const pos = editor.getPosition();
            console.log(pos);
            if (pos) setState("contents", state.currentFile, "lastCursorPos", {
                lineNumber: pos.lineNumber,
                column: pos.column
            });
            recompileFile(state.currentFile, content);
        }
    });
};

export const setEditorText = (text: string) => {
    const editorInstance = editor();
    if (editorInstance) {
        if (editorInstance.getValue() === text) return;
        editor()?.setValue(text);
    }
};

export const setEditorFile = (fileId: number) => {
    const content = state.contents[fileId];
    if (content) {
        const editorInstance = editor() as monaco.editor.IStandaloneCodeEditor;
        editorInstance.setValue(content.textContent || "");
        if (content.lastCursorPos) {
            editorInstance.setPosition(content.lastCursorPos);
            editorInstance.focus();
        }
    }
};