import { setEditorSelection } from "@state/editor";
import { Range } from "@types";
import { ParentProps } from "solid-js";


export const Navigatable = (props: ParentProps<{
    range: Range
}>) => {
    return <div class="p-1 cursor-pointer hover:bg-neutral-600 hover:bg-opacity-10 rounded-lg" onClick={() => {
        setEditorSelection(props.range.start, props.range.end);
    }}>
        {props.children}
    </div>;
};