import { Panel } from "../../Common/Panel";
import { createComponentFromItem, FMItem } from "./item";


export const FileManager = (props: { 
    files: FMItem[]
}) => {
    return <Panel text="Files" collapsable>
        <div class="pt-2">
            {props.files.map(f => createComponentFromItem(f))}
        </div>
    </Panel>;
};