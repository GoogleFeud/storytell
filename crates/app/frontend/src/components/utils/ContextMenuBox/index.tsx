/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { createSignal, JSXElement, onMount, ParentProps } from "solid-js";


export const ContextMenuBox = (props: ParentProps<{
    menu: JSXElement
}>) => {
    const [active, setActive] = createSignal<[number, number]>();

    onMount(() => {
        document.addEventListener("mousedown", () => {
            setActive();
        });
    });

    return <>
        <div onContextMenu={(ev) => {
            ev.preventDefault();
            setActive([ev.clientX, ev.clientY]);
        }}>
            {props.children}
            {active() && <div class="absolute z-[10000]" style={{
                left: active()![0] + "px",
                top: active()![1] + "px"
            }}>
                {props.menu}
            </div>}
        </div>
    </>;
};