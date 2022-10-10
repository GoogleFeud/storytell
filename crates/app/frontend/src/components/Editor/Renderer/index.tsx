
import { state } from "@state/index";
import { getActivePanelContents } from "@state/panel";
import { incrementIndex, setJoinNext } from "@state/renderer";
import { ASTHeader } from "@types";
import { createEffect, createSignal, For, untrack } from "solid-js";
import { renderBlock } from "./rendered";

export const Renderer = () => {
    const [activeAST, setActiveAST] = createSignal<ASTHeader | -1>();

    createEffect(() => {
        const active = getActivePanelContents();
        if (active) setActiveAST(active.compiledContent || -1);
    });

    return <div class="h-full"  onClick={() => incrementIndex()}>
        {activeAST() === -1 ? "Fix the errors!" : activeAST() === undefined ? "Select a file!" : <div>
            <p class="text-[24px] p-4 pl-5">{(activeAST() as ASTHeader).title}</p>
            <div class="p-2 pl-8 text-[14px] select-none">
                <For each={(activeAST() as ASTHeader).children.slice(0, state.renderer.currentIndex)}>{(block) => {
                    const rendered = renderBlock(block);
                    if (rendered) return <>
                        {rendered}
                        {untrack(() => state.renderer.joinNext === false ? <br /> : setJoinNext(false))}
                    </>;
                    else return undefined;
                }
                }</For>
            </div>
        </div>}
    </div>;
};