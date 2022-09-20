import { JSX, ParentProps } from "solid-js";


export const ScrollBox = (props: ParentProps<JSX.IntrinsicElements["div"]>) => {
    return <div class="overflow-auto invisible hover:visible focus:visible h-full w-full custom-scrollbar">
        <div class={`visible h-full w-full ${props.class}`}>
            {props.children}
        </div>
    </div>;
};