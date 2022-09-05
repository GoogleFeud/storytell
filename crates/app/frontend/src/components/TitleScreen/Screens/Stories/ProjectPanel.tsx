import { deleteProject, editProject, openProject, setModal } from "../../../../state";
import { Project } from "../../../../types";
import { EditIcon } from "../../../Icons/edit";
import { TrashIcon } from "../../../Icons/trash";
import { AreYouSureModal } from "../../../utils/Modal/AreYouSureModal";
import { ModifyProjectModal } from "../../../utils/Modal/ModifyProjectModal";

export const ProjectPanel = (props: {
    project: Project
}) => {
    return <div class="hover:scale-[1.008] transition" onClick={() => openProject(props.project)}>
        <div class="w-[50vw] h-[89px] border border-neutral-700 drop-shadow-lg rounded flex flex-col gap-3 cursor-pointer px-[18px] py-[12px] animate-[scale-in-center_0.2s_cubic-bezier(0.250,_0.460,_0.450,_0.940)_both]">
            <div class="flex justify-between items-center">
                <p class="text-[20px]">{props.project.metadata.name}</p>
                <div class="flex gap-4 text-neutral-400">
                    <div onClick={() => {
                        setModal(<ModifyProjectModal isUpdate={props.project} onFinish={(name, desc) => {
                            if (!name) return;
                            setModal();
                            editProject(props.project.metadata.name, name, desc);
                        }} />);
                    }}>
                        <EditIcon />
                    </div>
                    <div onClick={() => {
                        setModal(<AreYouSureModal text={`Do you want to delete "${props.project.metadata.name}" forever?`} onYes={() => deleteProject(props.project.metadata.name)} />);
                    }}><TrashIcon /></div>
                </div>
            </div>
            <div>
                <p class="text-[14px]">{props.project.metadata.description}</p>
            </div>
        </div>
    </div>;
};