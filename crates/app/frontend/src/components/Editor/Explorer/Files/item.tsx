import { children, createSignal } from "solid-js";
import { ArrowDownIcon } from "../../../Icons/arrowDown";
import { ArrowRightIcon } from "../../../Icons/arrowRight";
import { FileIcon } from "../../../Icons/file";

export interface FMItem {
    name: string,
    path: string,
    children?: Array<FMItem>
}

export const FileManagerFolder = (props: {
    item: FMItem
}) => {
    const [collapsed, setCollapsed] = createSignal(true);
    const realChildren = children(() => props.item.children?.map(c => createComponentFromItem(c)));
    return <div class="flex flex-col gap-1 ml-1">
        <div class="flex items-center gap-2 cursor-pointer" onClick={() => setCollapsed(!collapsed())}>
            {collapsed() ? <ArrowRightIcon size="13px" /> : <ArrowDownIcon size="12px" />}
            <p class={`text-[13px] text-neutral-400 ${collapsed() && "hover:text-neutral-200"}`}>{props.item.name}</p>
        </div>
        <div class="flex flex-col border-l border-neutral-700 pl-1 ml-1">
            {!collapsed() && realChildren}
        </div>
    </div>;
};

export const FileManagerFile = (props: {
    item: FMItem
}) => {
    return <div class="flex gap-2 p-1 items-center cursor-pointer">
        <FileIcon size="13px" />
        <p class="text-[13px] text-neutral-400 hover:text-neutral-200">{props.item.name}</p>
    </div>;
};

export const createComponentFromItem = (item: FMItem) => {
    return item.children ? <FileManagerFolder item={item} /> : <FileManagerFile item={item} />;
};