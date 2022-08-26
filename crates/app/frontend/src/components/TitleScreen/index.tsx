import { For, onMount } from "solid-js";
import { createProject, loadProjects, state } from "../../state";
import { ProjectPanel } from "./ProjectPanel";

export const TitleScreen = () => {
    
    onMount(async () => {
        await loadProjects();
    });

    return <div class="flex min-h-full">
        <div class="p-10 bg-[#E4DCCF] flex flex-col gap-28">
            <div>
                <img src="./assets/images/book.png" height="164px" width="152px" />
                <p class="text-[40px]">Storytell</p>
            </div>
            <div class="flex flex-col gap-[38px]">
                <div class="bg-[#F0EBE3] p-1 mr-[-90px] cursor-pointer">
                    <p class="text-[20px]">Stories</p>
                </div>
                <div>
                    <p class="text-[20px] p-1 cursor-pointer">Settings</p>
                </div>
                <div>
                    <p class="text-[20px] p-1 cursor-pointer">Guides</p>
                </div>
            </div>
        </div>
        <div class="w-full pt-[24px] pl-[32px] flex flex-col gap-28">
            <div class="flex gap-8">
                <div class="rounded-lg bg-[#7D9D9C] shadow-md text-[20px] text-white px-[26px] py-[11px] h-[46px] flex justify-center items-center cursor-pointer hover:scale-[1.008] transition" onClick={() => {
                    createProject("Untitled", "Some description!");
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