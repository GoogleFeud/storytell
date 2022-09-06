/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { JSXElement } from "solid-js";


export const VerticalResize = (props: {
    children: [JSXElement, JSXElement],
    minHTop?: string,
    minHBottom?: string,
    defaultHTop?: string
}) => {
    let topPane: HTMLDivElement|undefined;

    return <div class="w-full h-full flex flex-col">
        <div ref={topPane} style={{ 
            "min-height": props.minHTop,
            "height": props.defaultHTop
        }}>
            {props.children[0]}
        </div>
        <div class="relative" style={{
            "min-height": props.minHBottom
        }}>
            {props.children[1]}
            <div class="cursor-row-resize absolute top-0 left-0 w-full border border-neutral-700" onMouseDown={(ev) => {
                const topPaneDimensions = topPane!.getBoundingClientRect();

                const onMouseMove = (e: MouseEvent) => {
                    const newY = topPaneDimensions.height - (ev.y - e.y);
                    topPane!.style.height = newY + "px";
                };

                const onMouseLeave = () => {
                    window.removeEventListener("mousemove", onMouseMove);
                    window.removeEventListener("mouseup", onMouseLeave);
                };
                
                window.addEventListener("mousemove", onMouseMove);
                window.addEventListener("mouseup", onMouseLeave);
            }} />
        </div>
    </div>;
};