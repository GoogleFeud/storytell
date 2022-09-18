import { Accessor, For } from "solid-js";
import { removePanel, setActivePanel, state } from "../../../state";
import { Panel } from "../../../types";
import { FileIcon } from "../../Icons/file";
import { PinIcon } from "../../Icons/pin";
import { XIcon } from "../../Icons/x";
import { MonacoEditor } from "./monaco";
import { WelcomePanel } from "./panels/welcome";

export const MinimizedPanel = (props: {
    panel: Panel,
    id: Accessor<number>
}) => {
    return <div class={`flex justify-between items-center p-1 w-[142px] cursor-pointer ${state.activePanel === props.id() && "border-b border-[#6d4c41]"}`} onClick={() => {
        setActivePanel(props.id());
    }}>
        <div class="flex gap-1 items-center">
            {props.panel.icon || <FileIcon size="12px" />}
            <p class="text-[12px] text-ellipsis overflow-hidden whitespace-nowrap">{props.panel.name}</p>
        </div>
        {(console.log(props.id(), state.activePanel), "")}
        {props.panel.pinned ? <PinIcon class="bg-neutral-700 p-1 rounded" size="12px" /> : <XIcon class="hover:bg-neutral-700 p-1 rounded transition" size="12px" onClick={() => removePanel(props.id())} />}
    </div>;
};

export const TextEditor = () => {
    return <div class="h-full w-full flex flex-col">
        <div class="w-full h-[28px] flex border-b border-neutral-700">
            <For each={state.openPanels}>{(panel, ind) => {
                return <MinimizedPanel panel={panel} id={ind} />;
            }}</For>
        </div>
        {
            state.activePanel === undefined ? <WelcomePanel /> : <MonacoEditor />
        }
    </div>;
};