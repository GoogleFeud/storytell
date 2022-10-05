/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { Panel } from "@types";
import { state, setState } from ".";
import { openDirectoryRecursive, openFile, setCurrentFile } from "./file";


export const setActivePanel = (id: string|undefined) => {
    setState("activePanel", id);
};

export const getActivePanelContents = () => state.contents[state.openPanels.find(p => p.id === state.activePanel)?.fileId || -1];

export const openPanel = (id: Panel | string) => {
    const panel = typeof id === "string" ? state.openPanels.find(f => f.id === id) : id;
    if (!panel) return;
    if (panel.fileId) openFile(panel.fileId);
    else {
        setActivePanel(panel.id);
    }
};

export const createPanel = (panel: Panel) => {
    setState("openPanels", (p) => [panel, ...p]);
    setState("activePanel", panel.id);
};

export const removePanel = (id: string) => {
    const panel = state.openPanels.find(p => p.id === id);
    if (!panel) return;
    const panelId = state.openPanels.indexOf(panel);
    setState("openPanels", (p) => p.filter(p => p.id !== id));
    if (state.activePanel === id) {
        const selected = (state.openPanels[panelId] || state.openPanels[panelId - 1]);
        if (selected) {
            if (selected.fileId) {
                openFile(selected.fileId);
                openDirectoryRecursive(selected.fileId);
            } else {
                setActivePanel(selected.id);
            }
        } else {
            if (panel.fileId) setCurrentFile(undefined);
            setActivePanel(undefined);
        }
    }
};

export const setPanelPin = (id: string, pinned?: boolean) => {
    setState("openPanels", p => p.id === id, "pinned", pinned);
};