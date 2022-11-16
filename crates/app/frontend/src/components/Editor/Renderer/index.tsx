
import { ArrowCircleIcon } from "@icons/arrowCircle";
import { LongArrowLeftIcon } from "@icons/longArrowLeft";
import { state } from "@state/index";
import { activeHeader, activeBlock, resetToFirst, decrement, increment, activeBlockItems, setGlue, currentItem } from "@state/renderer";
import { ASTBlock, ASTBlockKind, ASTHeader, ASTInlineTextKind } from "@types";
import { For, untrack } from "solid-js";
import { renderBlock } from "./rendered";

export const Renderer = () => {
    return <div class="h-full">
        <div class="w-full border-b border-neutral-800 flex gap-2 items-center p-1.5">
            <ArrowCircleIcon size={"15px"} class="hover:bg-neutral-700 p-1 rounded transition" onClick={resetToFirst} />
            <LongArrowLeftIcon size={"15px"} class="hover:bg-neutral-700 p-1 rounded transition" onClick={decrement} />
        </div>
        <div class="h-full" onClick={() => increment()}>
            {activeBlock() === undefined ? "Select a file!" : <div>
                <p class="text-[24px] p-4 pl-5">{(activeHeader() as ASTHeader).title}</p>
                <div class="p-2 pl-8 text-[14px] select-none">
                    <For each={activeBlockItems()}>{(block) => {
                        // Paragraphs which consist of just a single join inline syntax get special treatment
                        if (isJoinLine(block)) return;
                        const rendered = renderBlock(block);
                        if (!rendered) {
                            increment();
                            return;
                        }
                        const lineBreak = untrack(() => {
                            if (!state.renderer.glueNext) {
                                if (isJoinLine(currentItem(0))) {
                                    increment(2);
                                    return;
                                }
                                return <br />;
                            }
                            setGlue(false);
                            increment();
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

const isJoinLine = (block?: ASTBlock) => {
    return block && block.kind === ASTBlockKind.Paragraph && !block.tail && block.parts.length === 1 && !block.parts[0].before && block.parts[0].text?.kind === ASTInlineTextKind.Join;
};