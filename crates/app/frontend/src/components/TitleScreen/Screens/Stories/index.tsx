import { For } from "solid-js";
import { setModal, state } from "../../../../state";
import { SearchIcon } from "../../../Icons/search";
import { Button } from "../../../Input/Button";
import { TextField } from "../../../Input/TextField";
import { CreateProjectModal } from "../../../utils/Modal/CreateProjectModal";
import { ProjectPanel } from "./ProjectPanel";

export const StoriesScreen = () => {
    return <div class="flex flex-col gap-20">
        <div class="flex justify-between">
            <h3 class="text-3xl">Stories</h3>
            <div class="flex flex-col gap-8 pr-8">
                <div class="flex gap-4">
                    <Button text="Import" />
                    <Button text="Create" primary onClick={async () => {
                        setModal(<CreateProjectModal />);
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