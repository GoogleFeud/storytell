import { createMemo, createSignal, JSX } from "solid-js";
import { sortFileList } from ".";
import { renameBlob, deleteBlob, createBlob, setOpenDirectory, setCreatingChildInDirectory, openFile, setCurrentFile } from "@state/file";
import { File, BlobType } from "../../../../types";
import { ArrowDownIcon } from "@icons/arrowDown";
import { ArrowRightIcon } from "@icons/arrowRight";
import { FileIcon } from "@icons/file";
import { ContextMenuBox } from "@utils/ContextMenuBox";
import { Input } from "@utils/Input";
import { ContextMenu } from "../../Common/ContextMenu";
import { state } from "@state/index";
import { ExclamationCircleIcon } from "@icons/exclamationCircle";

export const FileManagerInput = (props: JSX.InputHTMLAttributes<HTMLInputElement> & {
    value?: string,
    parent?: number,
    onExit?: (value: string) => void
}) => {
    return <Input {...props} type="text" class="text-[11.5px] outline-none bg-neutral-700 border border-neutral-600 w-full" ref={(ev) => setTimeout(() => ev.select(), 0)} validator={(val) => {
        if (val && !val.match(/^(?!\.)(?!com[0-9]$)(?!con$)(?!lpt[0-9]$)(?!nul$)(?!prn$)[^|*?\\:<>/$"]*[^.|*?\\:<>/$"]+$/)) return "Invalid file / folder name.";
        const parent = (props.parent ? state.fileExplorer.blobs[props.parent].children : state.fileExplorer.global) as number[];
        for (const child of parent) {
            if (state.fileExplorer.blobs[child].name === val) return "A file or folder with this name already exists.";
        }
        return;
    }} />;
};

export const FileManagerFolder = (props: {
    item: File,
}) => {
    const realChildren = createMemo(() => sortFileList((props.item.children as number[]).map(c => state.fileExplorer.blobs[c] as File)).map(c => createComponentFromItem(c)));
    const [isRenaming, setRenaming] = createSignal();
    return <div class="flex flex-col gap-1 ml-0.5">
        <ContextMenuBox menu={<ContextMenu commands={[
            {
                name: "New File...",
                execute: () => {
                    setOpenDirectory(props.item.id, true);
                    setCreatingChildInDirectory(props.item.id, BlobType.File);
                }
            },
            {
                name: "New Folder...",
                execute: () => {
                    setOpenDirectory(props.item.id, true);
                    setCreatingChildInDirectory(props.item.id, BlobType.Folder);
                }
            },
            {
                name: "Rename",
                execute: () => setRenaming(true)
            },
            {
                name: "Delete",
                execute: () => deleteBlob(props.item)
            }
        ]} />}>
            <div class={`flex items-center gap-2 cursor-pointer p-0.5 ${state.currentFile === props.item.id ? "w-full bg-[#6d4c41] text-white" : ""}`} onClick={(ev) => {
                setOpenDirectory(props.item.id, !props.item.isOpen);
                setCurrentFile(props.item.id);
                ev.stopPropagation();
            }}>
                {props.item.isOpen ? <ArrowDownIcon size="10px" /> : <ArrowRightIcon size="11px" />}
                {isRenaming() ? <FileManagerInput value={props.item.name} parent={props.item.parent} onExit={(newName) => {
                    if (newName) renameBlob(props.item, newName);
                    setRenaming();
                }} /> : <p class={`text-[12px] text-neutral-400 text-ellipsis overflow-hidden whitespace-nowrap ${!props.item.isOpen && "hover:text-neutral-200"}`}>{props.item.name}</p>}
            </div>
        </ContextMenuBox>
        <div class="flex flex-col border-l border-neutral-700 pl-1 ml-1.5">
            {props.item.isOpen && realChildren}
            {props.item.isCreating && <FileManagerCreating isFolder={props.item.isCreating === BlobType.Folder} parent={props.item.id} onEnd={() => setCreatingChildInDirectory(props.item.id)} />}
        </div>
    </div>;
};

export const FileManagerFile = (props: {
    item: File
}) => {
    const [isRenaming, setRenaming] = createSignal();
    return <ContextMenuBox menu={<ContextMenu commands={[
        {
            name: "Rename",
            execute: () => setRenaming(true)
        },
        {
            name: "Delete",
            execute: () => deleteBlob(props.item)
        }
    ]} />}>
        <div class={`flex gap-2 p-0.5 items-center cursor-pointer ${state.currentFile === props.item.id ? "w-full bg-[#6d4c41]" : ""}`} onClick={(ev) => {
            openFile(props.item.id);
            ev.stopPropagation();
        }}>
            <FileIcon size="12px" />
            {isRenaming() ? <FileManagerInput value={props.item.name} parent={props.item.parent} onExit={(newName) => {
                if (newName) renameBlob(props.item, newName);
                setRenaming();
            }} /> : <div class="flex justify-between items-center w-full">
                <p class={`text-[12px] text-ellipsis overflow-hidden whitespace-nowrap ${state.contents[props.item.id]?.diagnostics ? "text-red-500" : state.currentFile === props.item.id ? "text-white" : "text-neutral-400 hover:text-neutral-200"}`}>{props.item.name}</p>
                {state.contents[props.item.id]?.diagnostics && <div class="text-red-500 pr-2 text-[12px] flex gap-1 items-center">
                    <ExclamationCircleIcon size="12px" />
                    <p>{state.contents[props.item.id].diagnostics?.length}</p>
                </div>}
            </div>}
        </div>
    </ContextMenuBox>;
};

export const FileManagerCreating = (props: {
    isFolder?: boolean,
    parent?: number,
    onEnd?: () => void
}) => {
    const isFolder = !!props.isFolder;
    return <div>
        <div class="flex gap-2 p-0.5 items-center cursor-pointer">
            {isFolder ? <ArrowRightIcon size="13px" />  : <FileIcon size="13px" />}
            <FileManagerInput parent={props.parent} onExit={(newName) => {
                props.onEnd?.();
                if (newName) createBlob(newName, isFolder, props.parent);
            }} />
        </div>
    </div>;
};

export const createComponentFromItem = (item: File) => {
    return item.children ? <FileManagerFolder item={item} /> : <FileManagerFile item={item} />;
};