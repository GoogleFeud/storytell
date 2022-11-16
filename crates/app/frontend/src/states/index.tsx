import { invoke } from "@tauri-apps/api";
import { appWindow } from "@tauri-apps/api/window";
import { Panel, Project, FileContents, FileDiagnostic, Pages, File, RawFileContents, ASTBlock } from "@types";
import { JSXElement } from "solid-js";
import { createStore } from "solid-js/store";
import { createModel } from "./editor";

export const [state, setState] = createStore<{
    projects: Project[],
    modal?: JSXElement,
    currentProject?: Project,
    currentFile?: number,
    fileExplorer: {
        blobs: Record<number, File>,
        global: number[],
    },
    openPanels: Panel[],
    activePanel?: string,
    contents: Record<number, FileContents>,
    diagnostics: FileDiagnostic[],
    currentPage: Pages,
    renderer: {
        glueNext?: boolean,
        blocks: {
            children: ASTBlock[],
            index: number
        }[]
    }
}>({
    projects: [],
    fileExplorer: {
        blobs: {},
        global: []
    },
    diagnostics: [],
    contents: [],
    openPanels: [],
    currentPage: Pages.TitleScreen,
    renderer: {
        blocks: []
    }
});

appWindow.onCloseRequested(() => {
    saveData();
});

export const setModal = (modal?: JSXElement) => {
    setState("modal", modal);
};

export const initCompiler = async (projectId: string) : Promise<number|undefined> => {
    const result = JSON.parse(await invoke<string>("init_compiler", {projectId})) as {
        fileExplorer: {
            blobs: Record<string, File>,
            global: number[],
        },
        contents: RawFileContents[],
        openFolders: number[],
        pinnedPanels: number[],
        openPanels: number[],
        lastOpen?: number
    };
    for (const openFolder of result.openFolders) result.fileExplorer.blobs[openFolder].isOpen = true;
    setState("fileExplorer", result.fileExplorer);

    const contents: Record<number, FileContents> = {};
    for (const content of result.contents) {
        contents[content.id] = {
            model: createModel(content.id, content),
            compiledContent: content.compiledContent,
            diagnostics: content.diagnostics.length ? content.diagnostics : undefined
        };
    }
    setState("contents", contents);

    const openPanels = [];
    for (const fileId of result.openPanels) {
        openPanels.push({
            name: result.fileExplorer.blobs[fileId].name,
            id: fileId.toString(),
            fileId: fileId,
            pinned: result.pinnedPanels.includes(fileId)
        });
    }
    setState("openPanels", openPanels);

    return result.lastOpen;
};

export const saveData = () => {
    invoke<string>("save_data", {
        openPanels: state.openPanels.filter(p => p.fileId).map(p => p.fileId),
        pinnedPanels: state.openPanels.filter(p => p.pinned && p.fileId).map(p => p.fileId),
        openFolders: Object.values(state.fileExplorer.blobs).filter(p => p.children && p.isOpen).map(p => p.id),
        lastOpen: state.activePanel && state.fileExplorer.blobs[+state.activePanel] ? +state.activePanel : undefined
    });
};