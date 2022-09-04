import { For } from "solid-js";
import { createProject, setModal, state } from "../../../../state";
import { ErrorModal } from "../../../utils/ErrorModal";
import { ProjectPanel } from "./ProjectPanel";

export const ActionButton = (props: {
    text: string,
    onClick?: () => void,
    primary?: boolean
}) => {
    return <div class={`rounded ${props.primary ? "bg-[#6d4c41]" : "bg-[#404040]"} text-white text-[14px] px-4 py-2 flex justify-center items-center cursor-pointer min-w-[90px]`} onClick={props.onClick}>
        {props.text}
    </div>;
};

export const StoriesScreen = () => {
    return <div class="flex flex-col gap-28">
        <div class="flex justify-between items-center">
            <h3 class="text-3xl">Stories</h3>
            <div class="flex gap-4 pr-8">
                <ActionButton text="Import" />
                <ActionButton text="Create" primary onClick={async () => {
                    const created = await createProject("Untitled", "Some description!");
                    if (!created) setModal(<ErrorModal msg="Project with that name already exists." />);
                }}/>
            </div>
        </div>
        <div class="w-full flex flex-col gap-12">
            <For each={state.projects}>{(project) => <ProjectPanel project={project} />}</For>
        </div>
    </div>;
};