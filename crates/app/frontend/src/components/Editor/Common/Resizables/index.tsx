import { For, JSXElement, onMount } from "solid-js";
import Split from "split.js";

export const ResizablePane = (props: {
    children: JSXElement[],
    sizes?: number[],
    minSize?: number,
    vertical?: boolean
}) => {
    const refs: HTMLDivElement[] = [];

    onMount(() => {
        Split(refs, {
            direction: props.vertical ? "vertical" : "horizontal",
            sizes: props.sizes,
            minSize: props.minSize,
            gutterSize: 7
        });
    });

    return <div class={`w-full h-full flex ${props.vertical ? "flex-col" : "flex-row"}`}>
        <For each={props.children}>{(item, index) => {
            return <div ref={refs[index()]}>
                {item}
            </div>;
        }}</For>
    </div>;
};