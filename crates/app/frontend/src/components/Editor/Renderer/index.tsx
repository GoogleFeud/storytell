import { getActivePanelContents } from "@state/panel";
import { ASTHeader } from "@types";
import { createEffect, createSignal } from "solid-js";


export const Renderer = () => {
    const [activeAST, setActiveAST] = createSignal<ASTHeader | -1>();

    createEffect(() => {
        const active = getActivePanelContents();
        if (active) setActiveAST(active.compiledContent || -1);
    });

    return <div>
        {activeAST() === -1 ? "Fix the errors!" : activeAST() === undefined ? "Select a file!" : JSON.stringify(activeAST())}
    </div>;
};