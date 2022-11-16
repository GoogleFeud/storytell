import { ASTBlock } from "@types";
import { setState, state } from ".";

export const activeBlock = () => {
    return state.renderer.blocks[state.renderer.blocks.length - 1];
};

export const setInitialBlock = (children: ASTBlock[], index = 0) => {
    setState("renderer", "blocks", [{
        children,
        index
    }]);
};

export const activeHeader = () => {
    if (!state.currentFile) return undefined;
    return state.contents[state.currentFile]?.compiledContent;
};

export const currentItem = () => {
    const lastBlockInfo = state.renderer.blocks[state.renderer.blocks.length - 1];
    return lastBlockInfo.children[lastBlockInfo.index];
};

export const activeBlockItems = () => {
    const lastBlockInfo = state.renderer.blocks[state.renderer.blocks.length - 1];
    return lastBlockInfo.children.slice(0, lastBlockInfo.index);
};

export const increment = () => {
    setState("renderer", "blocks", state.renderer.blocks.length - 1, (block) => {
        if (block.index === block.children.length) return block;
        else return {...block, index: block.index + 1};
    });
};

export const decrement = () => {
    setState("renderer", "blocks", state.renderer.blocks.length - 1, "index", (prev) => prev - 1);
};

export const resetLast = () => {
    setState("renderer", "blocks", state.renderer.blocks.length - 1, "index", 0);
};

export const resetToFirst = () => {
    setState("renderer", "blocks", (blocklist) => [{
        index: 0,
        children: blocklist[0].children
    }]);
};

export const setGlue = (glue: boolean) => {
    setState("renderer", "glueNext", glue);
};