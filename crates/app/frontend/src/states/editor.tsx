import { Diagnostic, RawFileContacts } from "@types";
import * as monaco from "monaco-editor/esm/vs/editor/editor.api";
import { createSignal } from "solid-js";
import { setState, state } from ".";
import { openFile, recompileFile } from "./file";

export const [editor, setEditorState] = createSignal<monaco.editor.IStandaloneCodeEditor>();

export const setEditor = (editor: monaco.editor.IStandaloneCodeEditor) => {
    setEditorState(editor);
};

export const setEditorText = (text: string) => {
    const editorInstance = editor();
    if (editorInstance) {
        if (editorInstance.getValue() === text) return;
        editor()?.setValue(text);
    }
};

export const setEditorFile = async (fileId: number) => {
    if (!state.contents[fileId]) await openFile(fileId);
    const content = state.contents[fileId];
    if (content && content.model) {
        const editorInstance = editor() as monaco.editor.IStandaloneCodeEditor;
        editorInstance.setModel(content.model);
        if (content.viewState) editorInstance.restoreViewState(content.viewState);
    }
};

export const createModel = (fileId: number, contents: RawFileContacts) => {
    const model = monaco.editor.createModel(contents.textContent || "", "markdown");
    model.onDidChangeContent(async () => {
        const newDia = await recompileFile(fileId, model.getValue());
        setModelDiagnostics(model, newDia);
    });
    setModelDiagnostics(model, contents.diagnostics);
    return model;
};

export const saveFileModelState = (fileId: number | undefined) => {
    if (!fileId || !state.contents[fileId]) return;
    setState("contents", fileId, "viewState", editor()?.saveViewState() || undefined);
};

export const setModelDiagnostics = (model: monaco.editor.ITextModel, dias: Diagnostic[]|undefined) => {
    monaco.editor.setModelMarkers(model, "owner", (dias || []).map(dia => {
        const start = model.getPositionAt(dia.range.start);
        const end = model.getPositionAt(dia.range.end);
        return {
            message: dia.message,
            startLineNumber: start.lineNumber,
            startColumn: start.column,
            endLineNumber: end.lineNumber,
            endColumn: end.column,
            severity: monaco.MarkerSeverity.Error
        };
    }));
};