import { For } from "solid-js";
import { removePanel, setActivePanel, state } from "../../../state";
import { Panel } from "../../../types";
import { FileIcon } from "../../Icons/file";
import { PinIcon } from "../../Icons/pin";
import { XIcon } from "../../Icons/x";
import { MonacoEditor } from "./monaco";
import { WelcomePanel } from "./panels/welcome";

export const MinimizedPanel = (props: {
    panel: Panel
}) => {
    return <div class={`flex justify-between items-center p-1 w-[142px] cursor-pointer ${state.activePanel === props.panel.id && "border-b border-[#6d4c41]"}`} onClick={() => {
        setActivePanel(props.panel.id);
    }}>
        <div class="flex gap-1 items-center">
            {props.panel.icon || <FileIcon size="12px" />}
            <p class="text-[12px] text-ellipsis overflow-hidden whitespace-nowrap">{props.panel.name}</p>
        </div>
        {props.panel.pinned ? <PinIcon class="bg-neutral-700 p-1 rounded" size="12px" /> : <XIcon class="hover:bg-neutral-700 p-1 rounded transition" size="12px" onClick={(e) => {
            e.stopPropagation();
            removePanel(props.panel.id);
        }} />}
    </div>;
};

export const TextEditor = () => {
    return <div class="h-full w-full flex flex-col">
        <div class="w-full h-[28px] flex border-b border-neutral-700">
            <For each={state.openPanels}>{(panel) => {
                return <MinimizedPanel panel={panel} />;
            }}</For>
        </div>
        {
            state.activePanel === undefined ? <WelcomePanel /> : <MonacoEditor />
        }
    </div>;
};