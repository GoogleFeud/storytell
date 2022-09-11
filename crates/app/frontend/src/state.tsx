import { invoke } from "@tauri-apps/api";
import { JSXElement } from "solid-js";
import { createStore } from "solid-js/store";
import { FileDiagnostic, Pages, Project, File } from "./types";

export const [state, setState] = createStore<{
    projects: Project[],
    modal?: JSXElement,
    currentProject?: Project,
    currentFile?: string,
    files: File[],
    diagnostics: FileDiagnostic[],
    currentPage: Pages
}>({
    projects: [],
    files: [],
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
        files: File[],
        paths: unknown,
        diagnostics: FileDiagnostic[]
    };
    setState("files", result.files);
    setState("diagnostics", result.diagnostics);
};

export const setCurrentFile = (file: File) => setState("currentFile", file.path);

export const renameFile = async (filePath: string, name: string) => {
    const newPath = await invoke<string>("rename_file", { path: filePath, name });
    setState("files", f => f.path === filePath, (f) => ({...f, name, path: newPath}));
};