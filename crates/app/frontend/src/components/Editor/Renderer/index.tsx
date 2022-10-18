
import { ArrowCircleIcon } from "@icons/arrowCircle";
import { LongArrowLeftIcon } from "@icons/longArrowLeft";
import { state } from "@state/index";
import { getActivePanelContents } from "@state/panel";
import { decrementIndex, incrementIndex, setIndex, setJoinNext } from "@state/renderer";
import { ASTHeader } from "@types";
import { createEffect, createSignal, For, untrack } from "solid-js";
import { renderBlock } from "./rendered";

export const Renderer = () => {
    const [activeAST, setActiveAST] = createSignal<ASTHeader | -1>();

    createEffect(() => {
        const active = getActivePanelContents();
        if (active) setActiveAST(active.compiledContent || -1);
    });

    const inc = () => {
        const ast = activeAST();
        if (!ast || ast === -1) return;
        if (state.renderer.currentIndex < ast.children.length) incrementIndex();
    };

    return <div class="h-full">
        <div class="w-full border-b border-neutral-800 flex gap-2 items-center p-1.5">
            <ArrowCircleIcon size={"15px"} class="hover:bg-neutral-700 p-1 rounded transition" onClick={() => setIndex(0)} />
            <LongArrowLeftIcon size={"15px"} class="hover:bg-neutral-700 p-1 rounded transition" onClick={() => {
                if (state.renderer.currentIndex !== 0) decrementIndex();
            }} />
        </div>
        <div class="h-full" onClick={inc}>
            {activeAST() === -1 ? "Fix the errors!" : activeAST() === undefined ? "Select a file!" : <div>
                {/*(console.log(state.renderer.currentIndex), "") */}
                <p class="text-[24px] p-4 pl-5">{(activeAST() as ASTHeader).title}</p>
                <div class="p-2 pl-8 text-[14px] select-none">
                    <For each={(activeAST() as ASTHeader).children.slice(0, state.renderer.currentIndex)}>{(block) => {
                        const rendered = renderBlock(block);
                        if (!rendered) {
                            inc();
                            return;
                        }
                        const lineBreak = untrack(() => {
                            if (!state.renderer.joinNext) return <br />;
                            setJoinNext(false);
                            inc();
                            return;
                        });
                        if (rendered) return <>
                            {rendered}
                            {lineBreak}
                        </>;
                        else return undefined;
                    }
                    }</For>
                </div>
            </div>}
        </div>
    </div>;
};