import { getActivePanelContents } from "@state/panel";
import { ASTHeader } from "@types";
import { createEffect, createSignal, For } from "solid-js";
import { Navigatable } from "./navigatable";


export const Renderer = () => {
    const [activeAST, setActiveAST] = createSignal<ASTHeader | -1>();

    createEffect(() => {
        const active = getActivePanelContents();
        if (active) setActiveAST(active.compiledContent || -1);
    });

    return <div class="h-full">
        {activeAST() === -1 ? "Fix the errors!" : activeAST() === undefined ? "Select a file!" : <div class="p-2 pl-6">
            <p class="text-[24px] pb-4">{(activeAST() as ASTHeader).title}</p>
            <div class="flex flex-col gap-2">
                <For each={(activeAST() as ASTHeader).children}>{(block) => {
                    return <Navigatable range={block.range}>
                    Something!
                    </Navigatable>;
                }}</For>
            </div>
        </div>}
    </div>;
};