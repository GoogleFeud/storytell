/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { invoke } from "@tauri-apps/api";
import { File, BlobType, Diagnostic, RawFileContents } from "@types";
import { state, setState } from ".";
import { saveFileModelState, setEditorFile } from "./editor";
import { createPanel, removePanel, setActivePanel } from "./panel";

export const setCurrentFile = async (fileId?: number) => {
    setState("currentFile", fileId);
};

export const openFile = (fileId: number) => {
    // Save the previous open file's editor state
    if (state.activePanel) saveFileModelState(state.currentFile);
    setCurrentFile(fileId);
    setEditorFile(fileId);
    const file = state.fileExplorer.blobs[fileId];
    // Create a panel for the file or open an existing one
    if (!state.openPanels.some(p => p.fileId === fileId)) createPanel({
        id: fileId.toString(),
        name: file.name,
        fileId
    });
    else setActivePanel(fileId.toString());
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
    removePanel(file.id.toString());
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

export const recompileFile = async (fileId: number, content: string) : Promise<Diagnostic[]|undefined> => {
    const res = await JSON.parse(await invoke("recompile_file", {fileId, content})) as RawFileContents;
    setState("contents", fileId, "diagnostics", res.diagnostics.length ? res.diagnostics : undefined);
    setState("contents", fileId, "compiledContent", res.compiledContent);
    return res.diagnostics;
};