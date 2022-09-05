
import { createStore } from "solid-js/store";
import { createProject, setModal, state } from "../../../state";
import { XIcon } from "../../Icons/x";
import { Button } from "../../Input/Button";
import { TextFieldGroup } from "../../Input/TextField";
import { ModalBase } from "./base";


export const CreateProjectModal = () => {
    const [info, setInfo] = createStore<{name?: string, description?: string, error?: string}>({}); 
    return <ModalBase position="center" exitable>
        <div class="flex flex-col gap-12 w-[320px]">
            <div class="flex justify-between items-center">
                <h2 class="text-[18px] font-semibold">New story</h2>
                <div class="cursor-pointer" onClick={() => {
                    setModal();
                }}>
                    <XIcon />
                </div>
            </div>
            <div class="flex flex-col gap-4">
                <TextFieldGroup title="Story name" placeholder="Name..." error={info.error} onChange={(input) => {
                    if (state.projects.some(p => p.metadata.name === input)) return setInfo("error", "A project with this name already exists.");
                    setInfo("error", undefined);
                    setInfo("name", input);
                }} />
                <TextFieldGroup title="Story description" placeholder="Description..." onChange={(input) => {
                    setInfo("description", input);
                }} />
            </div>
            <Button text="Create" onClick={() => {
                if (!info.name) return;
                createProject(info.name, info.description || "");
                setModal();
            }} />
        </div>
    </ModalBase>;
};