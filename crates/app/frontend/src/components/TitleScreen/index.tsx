import { createSignal, JSXElement, onMount } from "solid-js";
import { loadProjects } from "../../state";
import { BookIcon } from "../Icons/book";
import { GearIcon } from "../Icons/gear";
import { GradCap } from "../Icons/gradcap";
import { Divider } from "../utils/Divider";
import { Screens } from "./Screens";

export const SidebarMenu = (props: {
    selected?: boolean,
    icon: JSXElement,
    text: string,
    onClick?: () => void
}) => {
    return <div class={`flex gap-4 cursor-pointer p-2 rounded-lg items-center w-[210px] ${props.selected ? "bg-neutral-700" : "bg-none"} transition-all`} onClick={props.onClick}>
        <div class="text-slate-300 pl-2">
            {props.icon}
        </div>
        <p class={`text-[16px] font-medium transition-all ${props.selected ? "text-white" : "text-neutral-400 hover:text-neutral-300"}`}>
            {props.text}
        </p>
    </div>;
};

export const TitleScreen = () => {
    const [activeScreen, setActiveScreen] = createSignal<number>(Screens.stories);
    
    onMount(async () => {
        await loadProjects();
    });

    return <div class="flex min-h-full">
        <div class="bg-[#242424] flex flex-col max-h-full min-w-[240px]">
            <div class="flex justify-center items-center gap-4 py-6">
                <img src="./assets/images/book.png" height="32px" width="32px" />
                <p class="text-[22px]">Storytell</p>
            </div>
            <Divider />
            <div class="flex flex-col justify-center items-center gap-3 py-4">
                <SidebarMenu icon={<BookIcon size="16px" />} text="Stories" selected={activeScreen() === Screens.stories} onClick={() => setActiveScreen(Screens.stories)} />
                <SidebarMenu icon={<GearIcon size="16px" />} text="Settings" selected={activeScreen() === Screens.settings} onClick={() => setActiveScreen(Screens.settings)} />
                <SidebarMenu icon={<GradCap size="16px" />} text="Guides" selected={activeScreen() === Screens.guides} onClick={() => setActiveScreen(Screens.guides)} />
            </div>
        </div>
        <div class="w-full pt-[24px] max-w-[1024px] pl-[32px]">
            {Screens.components[activeScreen()]}
        </div>
    </div>;
};