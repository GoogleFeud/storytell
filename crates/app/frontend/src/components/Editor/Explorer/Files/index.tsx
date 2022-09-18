/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { createSignal } from "solid-js";
import { isCurrentFolder, setCreatingChildInDirectory, setCurrentFile, setOpenDirectory, state } from "../../../../state";
import { BlobType, File } from "../../../../types";
import { MinimizeFolderIcon } from "../../../Icons/minimizeFolder";
import { PlusFileIcon } from "../../../Icons/plusFile";
import { PlusFolderIcon } from "../../../Icons/plusFolder";
import { Panel } from "../../Common/Panel";
import { createComponentFromItem, FileManagerCreating } from "./item";

export const sortFileList = (files: File[]) => {
    return files.sort((a, b) => {
        // First folders, then files
        if (a.children && !b.children) return -1;
        else if (b.children && !a.children) return 1;
        else return a.name.localeCompare(b.name);
    });
};

export const FileManager = (props: { 
    files: File[]
}) => {
    const [isCreating, setIsCreating] = createSignal<BlobType>();
    return <Panel text="Files" collapsable options={<div class="flex gap-3 pr-1">
        <PlusFileIcon size={"12px"} onClick={(e) => {
            e.stopPropagation();
            if (isCurrentFolder()) {
                setCreatingChildInDirectory(state.currentFile!, BlobType.File);
                setOpenDirectory(state.currentFile!, true);
            }
            else setIsCreating(BlobType.File);
        }} />
        <PlusFolderIcon size={"12px"} onClick={(e) => {
            e.stopPropagation();
            if (isCurrentFolder()) {
                setCreatingChildInDirectory(state.currentFile!, BlobType.Folder);
                setOpenDirectory(state.currentFile!, true);
            }
            else setIsCreating(BlobType.Folder);
        }} />
        <MinimizeFolderIcon size={"12px"} onClick={(e) => {
            e.stopPropagation();
            for (const icon in state.fileExplorer.blobs) {
                setOpenDirectory(+icon, false);
            }
        }} />
    </div>}>
        <div class="pt-2 h-full select-none w-full" onClick={() => setCurrentFile()}>
            {sortFileList(props.files).map(f => createComponentFromItem(f))}
            {isCreating() && <FileManagerCreating isFolder={isCreating() === BlobType.Folder} onEnd={setIsCreating()} />}
        </div>
    </Panel>;
};