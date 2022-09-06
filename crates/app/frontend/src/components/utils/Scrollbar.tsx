import { ParentProps } from "solid-js";


export const ScrollBox = (props: ParentProps<Record<string, unknown>>) => {
    return <div class="overflow-auto invisible hover:visible focus:visible h-full">
        <div class="visible h-full">
            {props.children}
        </div>
    </div>;
};