import { For } from "solid-js";


export const ContextMenu = (props: {
    commands: {
        name: string,
        shortcut?: string,
        execute: () => void
    }[]
}) => {
    return <div class="bg-neutral-900 roundex-[4px] flex flex-col gap-2 py-2 w-[180px]">
        <For each={props.commands}>{(command) => {
            return <div class="flex justify-between items-center cursor-pointer hover:bg-neutral-800 hover:text-white p-1 px-4" onMouseDown={command.execute}>
                <p class="text-[12px] text-neutral-100">{command.name}</p>
                {command.shortcut && <p class="text-[12px] text-neutral-100">{command.shortcut}</p>}
            </div>;
        }}</For>
    </div>;
};