import { removePanel, setActivePanel, setPanelPin, state } from "../../../../state";
import { Panel } from "../../../../types";
import { FileIcon } from "../../../Icons/file";
import { PinIcon } from "../../../Icons/pin";
import { XIcon } from "../../../Icons/x";
import { ContextMenuBox } from "../../../utils/ContextMenuBox";
import { ContextMenu } from "../../Common/ContextMenu";

export const MinimizedPanel = (props: {
    panel: Panel
}) => {
    return <ContextMenuBox menu={<ContextMenu commands={[
        {
            name: props.panel.pinned ? "Unpin" : "Pin",
            execute: () => {
                setPanelPin(props.panel.id, !props.panel.pinned);
            }
        }
    ]} />}>
        <div class={`flex justify-between items-center p-1 w-[142px] cursor-pointer ${state.activePanel === props.panel.id && "border-b border-[#6d4c41]"}`} onClick={() => {
            setActivePanel(props.panel.id);
        }}>
            <div class="flex gap-1 items-center">
                {props.panel.icon || <FileIcon size="12px" />}
                <p class="text-[12px] text-ellipsis overflow-hidden whitespace-nowrap">{props.panel.name}</p>
            </div>
            {props.panel.pinned ? <PinIcon class="bg-neutral-700 p-1 rounded" size="12px" onClick={() => setPanelPin(props.panel.id, false)} /> : <XIcon class="hover:bg-neutral-700 p-1 rounded transition" size="12px" onClick={(e) => {
                e.stopPropagation();
                removePanel(props.panel.id);
            }} />}
        </div>
    </ ContextMenuBox>;
};