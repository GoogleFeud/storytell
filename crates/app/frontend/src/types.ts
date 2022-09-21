import { JSXElement } from "solid-js";
import * as monaco from "monaco-editor/esm/vs/editor/editor.api";

export interface Project {
    metadata: {
        name: string,
        description: string,
        id: string
    },
    directory: string,
    files_directory: string
}

export interface Range {
    start: number,
    end: number
}

export interface Diagnostic {
    range: Range,
    message: string
}

export interface FileDiagnostic {
    filename: string,
    diagnostics: Diagnostic[]
}

export interface File {
    name: string,
    id: number,
    parent?: number,
    children?: number[],
    isOpen?: boolean,
    isCreating?: BlobType
}

export interface RawFileContacts {
    textContent?: string,
    diagnostics?: Diagnostic[]
}

export interface FileContents {
    model?: monaco.editor.ITextModel,
    diagnostics?: Diagnostic[],
    viewState?: monaco.editor.ICodeEditorViewState,
}

export interface Panel {
    name: string,
    fileId?: number,
    id: string,
    icon?: JSXElement,
    pinned?: boolean
}

export const enum BlobType {
    File = 1,
    Folder
}

export const enum Pages {
    TitleScreen,
    Editor
}
