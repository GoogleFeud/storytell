/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { Accessor, children, JSXElement } from "solid-js";


export const HorizontalResize = (props: {
    children: [JSXElement, JSXElement],
    minWLeft?: number,
    minWRight?: number,
    maxWLeft?: number,
    defaultWLeft?: number
}) => {
    const childrenProps = children(() => props.children) as Accessor<[JSXElement, JSXElement]>;
    let leftPane: HTMLDivElement|undefined;

    return <div class="w-full h-full flex">
        <div class="h-full" ref={leftPane} style={{ 
            "min-width": props.minWLeft && `${props.minWLeft}px`,
            "max-width": props.maxWLeft && `${props.maxWLeft}px`,
            "width": props.defaultWLeft && `${props.defaultWLeft}px`
        }}>
            {childrenProps()[0]}
        </div>
        <div class="relative w-auto h-full" style={{
            "min-width": props.minWRight && `${props.minWRight}px`,
        }}>
            {childrenProps()[1]}
            <div class="cursor-col-resize absolute top-0 left-0 h-full border border-neutral-700" onMouseDown={(ev) => {
                const leftPaneDimensions = leftPane!.getBoundingClientRect();

                const onMouseMove = (e: MouseEvent) => {
                    const newX = ev.x - e.x;
                    console.log(newX, leftPaneDimensions.width - newX);
                    leftPane!.style.width = leftPaneDimensions.width - newX + "px";
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