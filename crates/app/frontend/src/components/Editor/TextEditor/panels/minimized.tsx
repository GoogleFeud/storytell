import { state } from "@state/index";
import { openPanel, removePanel, setPanelPin } from "@state/panel";
import { Panel } from "@types";
import { FileIcon } from "@icons/file";
import { PinIcon } from "@icons/pin";
import { XIcon } from "@icons/x";
import { ContextMenuBox } from "@utils/ContextMenuBox";
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
        <div class={`flex justify-between items-center p-1 w-[142px] cursor-pointer ${state.activePanel === props.panel.id ? "border-b border-[#6d4c41]" : ""}`} onClick={() => {
            openPanel(props.panel);
        }}>
            <div class="flex gap-1 items-center">
                {props.panel.icon || <FileIcon size="12px" />}
                <p class={`text-[12px] text-ellipsis overflow-hidden whitespace-nowrap max-w-[100px] select-none ${(props.panel.fileId && state.contents[props.panel.fileId]?.diagnostics) ? "text-red-500" : ""}`}>{props.panel.name}</p>
            </div>
            <div class="flex items-center gap-0.5">
                <p class="text-red-500 text-[12px]">{props.panel.fileId && state.contents[props.panel.fileId]?.diagnostics?.length}</p>
                {props.panel.pinned ? <PinIcon class="bg-neutral-700 p-1 rounded" size="12px" onClick={() => setPanelPin(props.panel.id, false)} /> : <XIcon class="hover:bg-neutral-700 p-1 rounded transition" size="12px" onClick={(e) => {
                    e.stopPropagation();
                    removePanel(props.panel.id);
                }} />}
            </div>
        </div>
    </ ContextMenuBox>;
};