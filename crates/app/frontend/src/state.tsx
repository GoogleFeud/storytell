/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { invoke } from "@tauri-apps/api";
import { JSXElement } from "solid-js";
import { createStore } from "solid-js/store";
import { FileDiagnostic, Pages, Project, File } from "./types";

export const [state, setState] = createStore<{
    projects: Project[],
    modal?: JSXElement,
    currentProject?: Project,
    currentFile?: number,
    fileExplorer: {
        blobs: Record<number, File>,
        global: number[],
    }
    diagnostics: FileDiagnostic[],
    currentPage: Pages
}>({
    projects: [],
    fileExplorer: {
        blobs: {},
        global: []
    },
    diagnostics: [],
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

export const setCurrentFile = (file: File) => setState("currentFile", file.id);

export const renameBlob = async (file: File, name: string) => {
    await invoke<string>("rename_blob", { id: file.id, name: file.children ? name : name + ".md" });
    setState("fileExplorer", "blobs", file.id, (f) => ({...f, name }));
};

export const createFile = async (name: string, parent?: number) => {
    const file = JSON.parse(await invoke<string>("create_file", { parent, name: name + ".md" })) as File;
    setState("fileExplorer", "blobs", file.id, file);
    if (parent !== undefined) setState("fileExplorer", "blobs", parent, "children", (c) => [...c!, file.id]);
    else setState("fileExplorer", "global", (g) => [...g, file.id]);
};

export const deleteBlob = async (file: File, parent?: number) => {
    await invoke("delete_blob", { id: file.id });
    const newBlobs = {...state.fileExplorer.blobs};
    deleteBlobsRecursive(file, newBlobs);
    setState("fileExplorer", "blobs", newBlobs);
    if (parent !== undefined) setState("fileExplorer", "blobs", parent, "children", (children) => children!.filter(f => f !== file.id));
    else setState("fileExplorer", "global", (s) => s.filter(g => newBlobs[g]));
};

export const setOpenDirectory = (folder: number, open: boolean) => {
    setState("fileExplorer", "blobs", folder, "isOpen", open);
};

const deleteBlobsRecursive = (file: File, blobs: Record<number, File>) => {
    if (file.children) {
        for (const child of file.children) {
            deleteBlobsRecursive(blobs[child], blobs);
        }
    }
    delete blobs[file.id];
};