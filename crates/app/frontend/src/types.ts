
export interface Project {
    metadata: {
        name: string,
        description: string,
        id: string
    },
    directory: string,
    files_directory: string
}

export const enum Pages {
    TitleScreen,
    Editor
}
