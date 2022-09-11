import { setCurrentFile } from "../../../../state";
import { File } from "../../../../types";
import { PlusFileIcon } from "../../../Icons/plusFile";
import { PlusFolderIcon } from "../../../Icons/plusFolder";
import { Panel } from "../../Common/Panel";
import { createComponentFromItem } from "./item";


export const FileManager = (props: { 
    files: File[]
}) => {
    return <Panel text="Files" collapsable options={<div class="flex gap-3 pr-1">
        <PlusFileIcon size={"14px"} onClick={(e) => {
            e.stopPropagation();
        }} />
        <PlusFolderIcon size={"14px"} />
    </div>}>
        <div class="pt-2 select-none">
            {props.files.sort((a, b) => {
                // First folders, then files
                if (a.children && !b.children) return -1;
                else if (b.children && !a.children) return 1;
                else return a.name.localeCompare(b.name);
            }).map(f => createComponentFromItem(f, setCurrentFile))}
        </div>
    </Panel>;
};