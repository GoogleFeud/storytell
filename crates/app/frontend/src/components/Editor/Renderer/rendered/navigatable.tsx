import { setEditorSelection } from "@state/editor";
import { Range } from "@types";
import { children, ParentProps } from "solid-js";


export const Navigatable = (props: ParentProps<{
    range: Range
}>) => {
    const astChildren = children(() => props.children);
    return <span class="p-0.5 cursor-pointer hover:bg-neutral-600 hover:bg-opacity-10 rounded-lg" onClick={(ev) => {
        ev.stopPropagation();
        setEditorSelection(props.range.start, props.range.end);
    }}>
        {astChildren()}
    </span>;
};