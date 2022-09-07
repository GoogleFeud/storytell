
export interface Project {
    metadata: {
        name: string,
        description: string
    },
    directory: string,
    files_directory: string
}

export const enum Pages {
    TitleScreen,
    Editor
}
