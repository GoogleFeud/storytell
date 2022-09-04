import { createSignal, For, JSXElement, onMount } from "solid-js";
import { setModal } from "../../state";
import { createProject, loadProjects, state } from "../../state";
import { BookIcon } from "../Icons/book";
import { GearIcon } from "../Icons/gear";
import { GradCap } from "../Icons/gradcap";
import { ErrorModal } from "../utils/ErrorModal";
import { ProjectPanel } from "./ProjectPanel";

export const SidebarMenu = (props: {
    selected?: boolean,
    icon: JSXElement,
    text: string,
    onClick?: () => void
}) => {
    return <div class={`flex gap-4 cursor-pointer p-2 rounded-lg items-center w-[220px] ${props.selected ? "bg-neutral-700" : "bg-none"} transition-all`} onClick={props.onClick}>
        <div class="text-slate-300 pl-2">
            {props.icon}
        </div>
        <p class={`text-[16px] font-medium transition-all ${props.selected ? "text-white" : "text-neutral-400 hover:text-neutral-300"}`}>
            {props.text}
        </p>
    </div>;
};

export const enum Screens {
    Stories,
    Settings, 
    Guides
}

export const TitleScreen = () => {
    const [activeScreen, setActiveScreen] = createSignal<Screens>(Screens.Stories);
    
    onMount(async () => {
        await loadProjects();
    });

    return <div class="flex min-h-full">
        <div class="bg-[#242424] flex flex-col max-h-full w-[340px]">
            <div class="flex justify-center items-center gap-4 py-6">
                <img src="./assets/images/book.png" height="32px" width="32px" />
                <p class="text-[22px]">Storytell</p>
            </div>
            <div class="border-b border-neutral-700" />
            <div class="flex flex-col justify-center items-center gap-3 py-4">
                <SidebarMenu icon={<BookIcon size="16px" />} text="Stories" selected={activeScreen() === Screens.Stories} onClick={() => setActiveScreen(Screens.Stories)} />
                <SidebarMenu icon={<GearIcon size="16px" />} text="Settings" selected={activeScreen() === Screens.Settings} onClick={() => setActiveScreen(Screens.Settings)} />
                <SidebarMenu icon={<GradCap size="16px" />} text="Guides" selected={activeScreen() === Screens.Guides} onClick={() => setActiveScreen(Screens.Guides)} />
            </div>
        </div>
        <div class="w-full pt-[24px] pl-[32px] flex flex-col gap-28">
            <div class="flex gap-8">
                <div class="rounded-lg bg-[#7D9D9C] shadow-md text-[20px] text-white px-[26px] py-[11px] h-[46px] flex justify-center items-center cursor-pointer hover:scale-[1.008] transition" onClick={async () => {
                    const created = await createProject("Untitled", "Some description!");
                    if (!created) setModal(<ErrorModal msg="Project with that name already exists." />);
                }}>
                    <p>Create</p>
                </div>
                <div class="rounded-lg bg-[#7D9D9C] shadow-md text-[20px] text-white px-[26px] py-[11px] h-[46px] flex justify-center items-center cursor-pointer hover:scale-[1.008] transition">
                    <p>Import</p>
                </div>
            </div>
            <div class="w-full flex flex-col gap-12">
                <For each={state.projects}>{(project) => <ProjectPanel project={project} />}</For>
            </div>
        </div>
    </div>;
};