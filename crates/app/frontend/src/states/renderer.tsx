import { setState, state } from ".";

export const setJoinNext = (value: boolean) => {
    setState("renderer", "joinNext", value);
};

export const setIndex = (value: number) => {
    setState("renderer", "currentIndex", value);
};

export const incrementIndex = () => {
    setState("renderer", "currentIndex", state.renderer.currentIndex + 1);
};

export const decrementIndex = () => {
    setState("renderer", "currentIndex", state.renderer.currentIndex - 1);
};