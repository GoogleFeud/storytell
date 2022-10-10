import { setState, state } from ".";

export const setJoinNext = (value: boolean) => {
    console.log("SETTING JOIN NEXT!");
    setState("renderer", "joinNext", value);
};

export const incrementIndex = () => {
    setState("renderer", "currentIndex", state.renderer.currentIndex + 1);
};