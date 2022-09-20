
import { createStore } from "solid-js/store";
import { setModal, state } from "@state/index";
import { Project } from "@types";
import { XIcon } from "../../Icons/x";
import { Button } from "../../Input/Button";
import { TextFieldGroup } from "../../Input/TextField";
import { ModalBase } from "./base";


export const ModifyProjectModal = (props: {
    isUpdate?: Project,
    onFinish: (name?: string, description?: string) => void
}) => {
    const [info, setInfo] = createStore<{name?: string, description?: string, error?: string}>({ name: props.isUpdate?.metadata.name }); 
    return <ModalBase position="center" exitable>
        <div class="flex flex-col gap-12 w-[320px]">
            <div class="flex justify-between items-center">
                <h2 class="text-[18px] font-semibold">{props.isUpdate ? "Edit" : "New"} story</h2>
                <div class="cursor-pointer" onClick={() => {
                    setModal();
                }}>
                    <XIcon />
                </div>
            </div>
            <div class="flex flex-col gap-4">
                <TextFieldGroup title="Story name" placeholder="Name..." value={props.isUpdate?.metadata.name || ""} error={info.error} onChange={(input) => {
                    if (props.isUpdate) {
                        const project = state.projects.find(p => p.metadata.name === input);
                        if (project && project !== props.isUpdate) return setInfo("error", "A project with this name already exists.");
                    } else if (state.projects.some(p => p.metadata.name === input)) return setInfo("error", "A project with this name already exists.");
                    setInfo("error", undefined);
                    setInfo("name", input);
                }} />
                <TextFieldGroup title="Story description" value={props.isUpdate?.metadata.description || ""} placeholder="Description..." onChange={(input) => {
                    setInfo("description", input);
                }} />
            </div>
            <Button primary text={props.isUpdate ? "Update" : "Create"} onClick={() => props.onFinish(info.name, info.description)} />
        </div>
    </ModalBase>;
};