
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
    children?: number[],
    isOpen?: boolean,
    isCreating?: BlobType
}

export const enum BlobType {
    File = 1,
    Folder
}

export const enum Pages {
    TitleScreen,
    Editor
}
