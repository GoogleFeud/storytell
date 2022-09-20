/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { state, setState } from ".";
import { openDirectoryRecursive } from "./file";


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