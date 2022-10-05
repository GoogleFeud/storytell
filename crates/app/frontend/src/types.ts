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

export interface RawFileContents {
    /**
     * Only present when provided by INIT_COMPILER command
     */
    id: number,
    textContent?: string,
    diagnostics: Diagnostic[],
    compiledContent?: ASTHeader
}

export interface FileContents {
    model?: monaco.editor.ITextModel,
    diagnostics?: Diagnostic[],
    viewState?: monaco.editor.ICodeEditorViewState,
    compiledContent?: ASTHeader
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

export interface ASTAttriute extends Node<false> {
    name: string,
    parameters: string[]
}

export interface Node<Attributes extends boolean = true> {
    range: Range,
    attributes: Attributes extends true ? ASTAttriute[] : undefined
}

export interface ASTHeader extends Node {
    title: string,
    canonicalTitle: string,
    childPaths: ASTHeader[],
    children: ASTBlock[],
}

export const enum ASTInlineTextKind {
    Bold,
    Italics,
    Underline,
    Code,
    Join,
    JavaScript
}

export const enum ASTBlockKind {
    Paragraph,
    CodeBlock,
    ChoiceGroup,
    Divert,
    Match
}

export interface ASTInlineText extends Node<false> {
    kind: ASTInlineTextKind,
    text?: string
}

export interface ASTTextPart {
    before: string,
    text: ASTInlineText
}

export interface ASTText extends Node<false> {
    parts: ASTTextPart[],
    tail: string
}

export interface ASTParagraph extends Node {
    kind: ASTBlockKind.Paragraph,
    parts: ASTTextPart[],
    tail: string
}

export interface ASTCodeBlock extends Node {
    kind: ASTBlockKind.CodeBlock,
    code: string,
    language: string
}

export interface ASTChoice extends Node {
    text: ASTText,
    children: ASTBlock[],
    condition?: {
        modifier: string,
        text: string
    }
}

export interface ASTChoiceGroup extends Node {
    kind: ASTBlockKind.ChoiceGroup,
    choices: ASTChoice[]
}

export interface ASTDivert extends Node {
    kind: ASTBlockKind.Divert
    path: string[]
}

export interface ASTMatch extends Node {
    condition: string,
    modifier?: string,
    arms: ASTChoice[],
    children: ASTBlock[]
}

export type ASTBlock = ASTParagraph | ASTCodeBlock | ASTChoiceGroup | ASTDivert | ASTMatch;