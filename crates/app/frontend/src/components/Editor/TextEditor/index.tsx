import { For } from "solid-js";
import { state } from "../../../state";
import { ScrollBox } from "../../utils/Scrollbar";
import { MonacoEditor } from "./monaco";
import { MinimizedPanel } from "./panels/minimized";
import { WelcomePanel } from "./panels/welcome";

export const TextEditor = () => {
    return <div class="h-full w-full flex flex-col">
        <div class="w-full flex border-b border-neutral-700">
            <ScrollBox class="flex">
                <For each={state.openPanels.filter(p => p.pinned)}>{(panel) => {
                    return <MinimizedPanel panel={panel} />;
                }}</For>
                <For each={state.openPanels.filter(p => !p.pinned)}>{(panel) => {
                    return <MinimizedPanel panel={panel} />;
                }}</For>
            </ScrollBox>
        </div>
        {
            state.activePanel === undefined ? <WelcomePanel /> : <MonacoEditor />
        }
    </div>;
};