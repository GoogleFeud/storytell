import { invoke } from "@tauri-apps/api";
import { JSXElement } from "solid-js";
import { createStore } from "solid-js/store";
import { Project } from "./types";

export const [state, setState] = createStore<{
    projects: Project[],
    modal?: JSXElement
}>({
    projects: []
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

export const editProject = async (oldName: string, name: string, description?: string) => {
    setState("projects", (p) => p.metadata.name === oldName, "metadata", {name, description: description || ""});
    invoke("edit_project", {oldName, name, description});
};

export const deleteProject = async (name: string) => {
    setState("projects", (p) => p.filter(p => p.metadata.name !== name));
    await invoke<string>("delete_project", {name});
};

export const setModal = (modal?: JSXElement) => {
    setState("modal", modal);
};