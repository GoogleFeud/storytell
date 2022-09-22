/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { invoke } from "@tauri-apps/api";
import { File, BlobType, RawFileContacts, Diagnostic } from "@types";
import { state, setState, saveData } from ".";
import { createModel, saveFileModelState, setEditorFile } from "./editor";
import { createPanel, removePanel, setActivePanel } from "./panel";

export const setCurrentFile = async (file?: File) => {
    if (file) {
        saveFileModelState(state.currentFile);
        setState("currentFile", file.id);
        if (!file.children) {
            setEditorFile(file.id);
            if (!state.openPanels.some(p => p.fileId === file.id)) createPanel({
                name: file.name,
                fileId: file.id,
                id: file.id.toString()
            });
            else {
                setActivePanel(file.id.toString());
            }
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

export const openDirectoryRecursive = (dir: number) => {
    const dirObj = state.fileExplorer.blobs[dir];
    if (dirObj.children) setState("fileExplorer", "blobs", dir, "isOpen", true);
    if (dirObj.parent) openDirectoryRecursive(dirObj.parent);
};

export const openFile = async (fileId: number) => {
    const res = await JSON.parse(await invoke("open_file", {fileId})) as RawFileContacts;
    setState("contents", fileId, {
        diagnostics: res.diagnostics,
        model: createModel(fileId, res)
    });
};

export const recompileFile = async (fileId: number, content: string) : Promise<Diagnostic[]|undefined> => {
    saveData();
    const res = await JSON.parse(await invoke("recompile_file", {fileId, content})) as RawFileContacts;
    setState("contents", fileId, "diagnostics", res.diagnostics);
    return res.diagnostics;
};