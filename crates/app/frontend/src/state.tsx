import { invoke } from "@tauri-apps/api";
import { createStore } from "solid-js/store";
import { Project } from "./types";

export const [state, setState] = createStore<{
    projects: Project[]
}>({
    projects: []
});

export const loadProjects = async (): Promise<Project[]> => {
    const projects = JSON.parse(await invoke<string>("list_projects"));
    setState("projects", projects);
    return state.projects;
};

export const createProject = async (name: string, description: string): Promise<Project|undefined> => {
    const project = JSON.parse(await invoke<string>("create_project", {name, description}));
    if (!project) return;
    setState("projects", (p) => [...p, project]);
    return project;
};

export const deleteProject = async (name: string) => {
    setState("projects", (p) => p.filter(p => p.metadata.name !== name));
    await invoke<string>("delete_project", {name});
};