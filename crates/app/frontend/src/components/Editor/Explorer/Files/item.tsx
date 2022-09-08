import { children, createSignal } from "solid-js";
import { state } from "../../../../state";
import { File } from "../../../../types";
import { ArrowDownIcon } from "../../../Icons/arrowDown";
import { ArrowRightIcon } from "../../../Icons/arrowRight";
import { FileIcon } from "../../../Icons/file";
import { ContextMenuBox } from "../../../utils/ContextMenuBox";
import { Input } from "../../../utils/Input";
import { ContextMenu } from "../../Common/ContextMenu";

export const FileManagerFolder = (props: {
    item: File,
    onSelect?: (file: File) => void 
}) => {
    const [collapsed, setCollapsed] = createSignal(true);
    const realChildren = children(() => props.item.children?.map(c => createComponentFromItem(c, props.onSelect)));
    return <div class="flex flex-col gap-1 ml-1">
        <div class={`flex items-center gap-2 cursor-pointer p-1 ${state.currentFile === props.item.path ? "w-full bg-[#6d4c41] text-white" : ""}`} onClick={() => {
            setCollapsed(!collapsed());
            props.onSelect?.(props.item);
        }}>
            {collapsed() ? <ArrowRightIcon size="13px" /> : <ArrowDownIcon size="12px" />}
            <p class={`text-[13px] text-neutral-400 ${collapsed() && "hover:text-neutral-200"}`}>{props.item.name}</p>
        </div>
        <div class="flex flex-col border-l border-neutral-700 pl-1 ml-1">
            {!collapsed() && realChildren}
        </div>
    </div>;
};

export const FileManagerFile = (props: {
    item: File,
    onSelect?: (file: File) => void
}) => {
    const [isRenaming, setRenaming] = createSignal();
    return <ContextMenuBox menu={<ContextMenu commands={[
        {
            name: "Rename",
            execute: () => setRenaming(true)
        },
        {
            name: "Delete",
            execute: () => console.log("Rename.")
        }
    ]} />}>
        <div class={`flex gap-2 p-1 items-center cursor-pointer ${state.currentFile === props.item.path ? "w-full bg-[#6d4c41] text-white" : ""}`} onClick={() => props.onSelect?.(props.item)}>
            <FileIcon size="13px" />
            {isRenaming() ? <Input type="text" class="text-[13px] outline-none bg-neutral-700 border border-neutral-600 w-full" value={props.item.name} ref={(ev) => setTimeout(() => ev.select(), 0)} onExit={() => {
                // Set name here in the state and send to the backend...
                setRenaming();
            }} /> : <p class="text-[13px] text-neutral-400 hover:text-neutral-200">{props.item.name.split(".")[0]}</p>}
        </div>
    </ContextMenuBox>;
};

export const createComponentFromItem = (item: File, onSelect?: (file: File) => void) => {
    return item.children ? <FileManagerFolder item={item} onSelect={onSelect} /> : <FileManagerFile item={item} onSelect={onSelect} />;
};