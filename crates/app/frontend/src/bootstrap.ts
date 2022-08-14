import { createStore } from "solid-js/store";

export interface Path {
    name: string,
    render: () => Promise<void>,
    children: Array<Path>
}

export interface ChoiceGroup {
    debugName?: string,
    choices: Array<Choice>
}

export interface Choice {
    text: string,
    group: ChoiceGroup,
    render: () => Promise<void>
}

export interface HistoryEntry {
    choiceMade?: Choice,
    capturedState: Record<string, any>
}

declare global {

    interface Window {
        pathStack: Array<Path>,
        storyHistory: Array<HistoryEntry>,
        paths: Record<string, Path>,
        state: Record<string, any>,
        jumpToPath(path: Array<string>, choice?: Choice): void,
        // Returns true if the choice has moved the paths
        makeChoice(group: ChoiceGroup): Promise<[Choice, boolean]>,
        resetData(): void,
    }

}


export const [state, setState] = createStore<{
    choiceSelection?: [ChoiceGroup, (selectedChoice: Choice) => Promise<void>],
    currentPath?: Path,
    textEntries: Array<string>
}>({
    textEntries: []
});

window.jumpToPath = (path: Array<string>, choice?: Choice) => {
    let finalPath = window.paths[path[0]];
    if (!finalPath) return;
    for (let i = 1; i < path.length; i++) {
        finalPath = finalPath.children[i];
        if (!finalPath) return;
    }
    window.storyHistory.push({
        choiceMade: choice,
        capturedState: saveState()
    });
};

window.makeChoice = (group: ChoiceGroup) => {
    return new Promise(res => {
        const currentPath = state.currentPath;
        setState("choiceSelection", [group, async (selected) => {
            await selected.render();
            res([selected, currentPath === state.currentPath]);
        }]);
    });
};

export function saveState(): Record<string, any> {
    return JSON.parse(JSON.stringify(window.state));
}