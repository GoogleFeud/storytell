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
        files: Record<number, File>,
        dirs: Record<number, File>,
        global: number[],
        lastId: number
    }
    diagnostics: FileDiagnostic[],
    currentPage: Pages
}>({
    projects: [],
    fileExplorer: {
        files: {},
        dirs: {},
        global: [],
        lastId: 0
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
            files: Record<number, File>,
            dirs: Record<number, File>,
            global: number[],
            lastId: number
        }
    };
    setState("fileExplorer", result.fileExplorer);
};

export const setCurrentFile = (file: File) => setState("currentFile", file.id);

export const renameFile = async (file: File, name: string) => {
    const newPath = await invoke<string>("rename_file", { id: file.id, name: file.children ? name : name + ".md" });
    if (file.children) {
        setState("fileExplorer", "dirs", file.id, (f) => ({...f, name, path: newPath }));
    }
    else setState("fileExplorer", "files", file.id, (f) => ({...f, name, path: newPath }));
};