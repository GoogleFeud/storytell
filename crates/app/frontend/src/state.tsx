/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { invoke } from "@tauri-apps/api";
import { JSXElement } from "solid-js";
import { createStore } from "solid-js/store";
import { FileDiagnostic, Pages, Project, File, BlobType, FileContents, Panel } from "./types";

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

export const loadProjects = async (): Promise<Project[]> => {
    const projects = JSON.parse(await invoke<string>("list_projects"));
    setState("projects", [...state.projects, ...projects]);
    return state.projects;
};

export const createProject = async (name: string, description: string): Promise<Project|undefined> => {
    const project = JSON.parse(await invoke<string>("create_project", {name, description}));
    if (!project) return;
    setState("projects", (p) => [...p, project]);
    return project;
};

export const editProject = async (id: string, name: string, description?: string) => {
    setState("projects", (p) => p.metadata.id === id, "metadata", {name, description: description || ""});
    invoke("edit_project", {id, name, description});
};

export const deleteProject = async (id: string) => {
    setState("projects", (p) => p.filter(p => p.metadata.id !== id));
    await invoke<string>("delete_project", {id});
};

export const setModal = (modal?: JSXElement) => {
    setState("modal", modal);
};

export const openProject = (project: Project) => {
    setState("currentProject", project);
    setState("currentPage", Pages.Editor);
};

export const initCompiler = async (projectId: string) => {
    const result = JSON.parse(await invoke<string>("init_compiler", {projectId})) as {
        fileExplorer: {
            blobs: Record<string, File>,
            global: number[],
        }
    };
    setState("fileExplorer", result.fileExplorer);
};

export const setCurrentFile = (file?: File, addToPanels = true) => {
    if (file) {
        setState("currentFile", file.id);
        if (!file.children && addToPanels) {
            if (!state.openPanels.some(p => p.fileId === file.id)) setState("openPanels", (p) => [{
                name: file.name,
                fileId: file.id,
                id: file.id.toString()
            }, ...p]);
            setState("activePanel", file.id.toString());
        }
    } else {
        setState("currentFile", undefined);
    }
};

export const getCurrentFile = () => state.currentFile && state.fileExplorer.blobs[state.currentFile];

export const isCurrentFolder = () => state.currentFile && !!state.fileExplorer.blobs[state.currentFile].children;

export const renameBlob = async (file: File, name: string) => {
    await invoke<string>("rename_blob", { id: file.id, name: file.children ? name : name + ".md" });
    setState("fileExplorer", "blobs", file.id, (f) => ({...f, name }));
    const filePanel = state.openPanels.findIndex(p => p.fileId === file.id);
    if (filePanel !== -1) setState("openPanels", filePanel, "name", name);
};

export const createBlob = async (name: string, isDir: boolean, parent?: number) => {
    const file = JSON.parse(await invoke<string>("create_blob", { parent, name: isDir ? name : name + ".md", dir: isDir })) as File;
    setState("fileExplorer", "blobs", file.id, file);
    if (parent) setState("fileExplorer", "blobs", parent, "children", (c) => [...c!, file.id]);
    else setState("fileExplorer", "global", (g) => [...g, file.id]);
};

export const deleteBlob = async (file: File) => {
    await invoke("delete_blob", { id: file.id });
    const newBlobs = {...state.fileExplorer.blobs};
    deleteBlobsRecursive(file, newBlobs);
    setState("fileExplorer", "blobs", newBlobs);
    if (file.parent) setState("fileExplorer", "blobs", file.parent, "children", (children) => children!.filter(f => f !== file.id));
    else setState("fileExplorer", "global", (s) => s.filter(g => newBlobs[g]));
    removePanel(file.id.toString());
};

const deleteBlobsRecursive = (file: File, blobs: Record<number, File>) => {
    if (file.children) {
        for (const child of file.children) {
            deleteBlobsRecursive(blobs[child], blobs);
        }
    }
    delete blobs[file.id];
};

export const refreshBlobs = async () => {
    const refreshed = JSON.parse(await invoke<string>("refresh_blobs")) as {
        blobs: Record<string, File>,
        global: number[],
    };
    for (const blob in state.fileExplorer.blobs) {
        refreshed.blobs[blob].isOpen = state.fileExplorer.blobs[blob].isOpen;
    }
    setState("fileExplorer", refreshed);
};

export const setOpenDirectory = (folder: number, open: boolean) => {
    setState("fileExplorer", "blobs", folder, "isOpen", open);
};

export const setCreatingChildInDirectory = (folder: number, type?: BlobType) => {
    setState("fileExplorer", "blobs", folder, "isCreating", type);
};

const openDirectoryRecursive = (dir: number) => {
    const dirObj = state.fileExplorer.blobs[dir];
    if (dirObj.children) setState("fileExplorer", "blobs", dir, "isOpen", true);
    if (dirObj.parent) openDirectoryRecursive(dirObj.parent);
};

export const setActivePanel = (id: string) => {
    setState("activePanel", id);
    const panel = state.openPanels.find(p => p.id === id)!;
    if (panel.fileId) {
        setState("currentFile", panel.fileId);
        openDirectoryRecursive(panel.fileId);
    }
};

export const removePanel = (id: string) => {
    const panel = state.openPanels.find(p => p.id === id);
    if (!panel) return;
    const panelId = state.openPanels.indexOf(panel);
    setState("openPanels", (p) => p.filter(p => p.id !== id));
    if (state.activePanel === id) {
        const selected = (state.openPanels[panelId] || state.openPanels[panelId - 1]);
        if (selected) {
            setState("activePanel", selected.id);
            if (selected.fileId) {
                setState("currentFile", selected.fileId);
                openDirectoryRecursive(selected.fileId);
            }
        } else {
            if (panel.fileId) setState("currentFile", undefined);
            setState("activePanel", undefined);
        }
    }
};

export const setPanelPin = (id: string, pinned?: boolean) => {
    setState("openPanels", p => p.id === id, "pinned", pinned);
};