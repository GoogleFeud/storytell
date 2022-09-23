import { invoke } from "@tauri-apps/api";
import { Panel, Project, FileContents, FileDiagnostic, Pages, File } from "@types";
import { JSXElement } from "solid-js";
import { createStore } from "solid-js/store";
import { openFile } from "./file";

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
    currentPage: Pages
}>({
    projects: [],
    fileExplorer: {
        blobs: {},
        global: []
    },
    diagnostics: [],
    contents: [],
    openPanels: [],
    currentPage: Pages.TitleScreen
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
        openFolders: number[],
        pinnedPanels: number[],
        openPanels: number[],
        lastOpen?: number
    };
    for (const openFolder of result.openFolders) {
        if (result.fileExplorer.blobs[openFolder]) result.fileExplorer.blobs[openFolder].isOpen = true;
    }
    setState("fileExplorer", result.fileExplorer);
    const openPanels = [];
    for (const fileId of result.openPanels) {
        if (!result.fileExplorer.blobs[fileId]) continue;
        await openFile(fileId);
        openPanels.push({
            name: result.fileExplorer.blobs[fileId].name,
            id: fileId.toString(),
            fileId: fileId,
            pinned: result.pinnedPanels.includes(fileId)
        });
    }
    setState("openPanels", openPanels);
    if (result.lastOpen && result.fileExplorer.blobs[result.lastOpen]) {
        await openFile(result.lastOpen);
        return result.lastOpen;
    }
    return;
};

export const saveData = () => {
    invoke<string>("save_data", {
        openPanels: state.openPanels.filter(p => p.fileId).map(p => p.fileId),
        pinnedPanels: state.openPanels.filter(p => p.pinned && p.fileId).map(p => p.fileId),
        openFolders: Object.values(state.fileExplorer.blobs).filter(p => p.children && p.isOpen).map(p => p.id),
        lastOpen: state.activePanel && state.fileExplorer.blobs[+state.activePanel] ? +state.activePanel : undefined
    });
};