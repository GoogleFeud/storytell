/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { createSignal } from "solid-js";
import { isCurrentFolder, refreshBlobs, setCreatingChildInDirectory, setCurrentFile, setOpenDirectory } from "@state/file";
import { state } from "@state/index";
import { BlobType, File } from "@types";
import { ArrowCircleIcon } from "@icons/arrowCircle";
import { MinimizeFolderIcon } from "@icons/minimizeFolder";
import { PlusFileIcon } from "@icons/plusFile";
import { PlusFolderIcon } from "@icons/plusFolder";
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
    return <Panel text="Files" collapsable options={<div class="flex gap-1 pr-1">
        <PlusFileIcon size={"11px"} class="hover:bg-neutral-700 p-1 rounded transition" onClick={(e) => {
            e.stopPropagation();
            if (isCurrentFolder()) {
                setCreatingChildInDirectory(state.currentFile!, BlobType.File);
                setOpenDirectory(state.currentFile!, true);
            }
            else setIsCreating(BlobType.File);
        }} />
        <PlusFolderIcon size={"11px"} class="hover:bg-neutral-700 p-1 rounded transition" onClick={(e) => {
            e.stopPropagation();
            if (isCurrentFolder()) {
                setCreatingChildInDirectory(state.currentFile!, BlobType.Folder);
                setOpenDirectory(state.currentFile!, true);
            }
            else setIsCreating(BlobType.Folder);
        }} />
        <ArrowCircleIcon size={"11px"} class="hover:bg-neutral-700 p-1 rounded transition" onClick={(e) => {
            e.stopPropagation();
            refreshBlobs();
        }} />
        <MinimizeFolderIcon size={"11px"} class="hover:bg-neutral-700 p-1 rounded transition" onClick={(e) => {
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