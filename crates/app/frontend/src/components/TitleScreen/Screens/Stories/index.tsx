import { For } from "solid-js";
import { createProject } from "@state/project";
import { SearchIcon } from "@icons/search";
import { Button } from "@input/Button";
import { TextField } from "@input/TextField";
import { ModifyProjectModal } from "@utils/Modal/ModifyProjectModal";
import { ProjectPanel } from "./ProjectPanel";
import { setModal, state } from "@state/index";

export const StoriesScreen = () => {
    return <div class="flex flex-col gap-20">
        <div class="flex justify-between">
            <h3 class="text-3xl">Stories</h3>
            <div class="flex flex-col gap-8 pr-8">
                <div class="flex gap-4">
                    <Button text="Import" />
                    <Button text="Create" primary onClick={async () => {
                        setModal(<ModifyProjectModal onFinish={(name, desc) => {
                            if (!name) return;
                            createProject(name, desc || "");
                            setModal();
                        }} />);
                    }}/>
                </div>
                <TextField placeholder="Search" icon={<SearchIcon size="14px" />} />
            </div>
        </div>
        <div class="w-full flex flex-col gap-8">
            <For each={state.projects}>{(project) => <ProjectPanel project={project} />}</For>
        </div>
    </div>;
};