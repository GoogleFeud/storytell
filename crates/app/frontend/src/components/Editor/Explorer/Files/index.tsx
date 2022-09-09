import { setCurrentFile } from "../../../../state";
import { File } from "../../../../types";
import { Panel } from "../../Common/Panel";
import { createComponentFromItem } from "./item";


export const FileManager = (props: { 
    files: File[]
}) => {
    return <Panel text="Files" collapsable>
        <div class="pt-2">
            {props.files.slice().sort((a, b) => {
                // First folders, then files
                if (a.children && !b.children) return -1;
                else if (b.children && !a.children) return 1;
                else return a.name.localeCompare(b.name);
            }).map(f => createComponentFromItem(f, setCurrentFile))}
        </div>
    </Panel>;
};