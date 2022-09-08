import { deleteProject, editProject, initCompiler, openProject, setModal } from "../../../../state";
import { Project } from "../../../../types";
import { EditIcon } from "../../../Icons/edit";
import { TrashIcon } from "../../../Icons/trash";
import { AreYouSureModal } from "../../../utils/Modal/AreYouSureModal";
import { ModifyProjectModal } from "../../../utils/Modal/ModifyProjectModal";

export const ProjectPanel = (props: {
    project: Project
}) => {
    return <div class="hover:scale-[1.008] transition">
        <div class="w-[50vw] h-[89px] border border-neutral-700 drop-shadow-lg rounded flex flex-col gap-3 cursor-pointer px-[18px] py-[12px] animate-[scale-in-center_0.2s_cubic-bezier(0.250,_0.460,_0.450,_0.940)_both]" onClick={() => {
            openProject(props.project);
            initCompiler(props.project.metadata.id);
        }}>
            <div class="flex justify-between items-center">
                <p class="text-[20px]">{props.project.metadata.name}</p>
                <div class="flex gap-4 text-neutral-400">
                    <div onClick={(ev) => {
                        setModal(<ModifyProjectModal isUpdate={props.project} onFinish={(name, desc) => {
                            if (!name) return;
                            setModal();
                            editProject(props.project.metadata.id, name, desc);
                        }} />);
                        ev.stopPropagation();
                    }}>
                        <EditIcon />
                    </div>
                    <div onClick={(ev) => {
                        setModal(<AreYouSureModal text={`Do you want to delete "${props.project.metadata.name}" forever?`} onYes={() => deleteProject(props.project.metadata.id)} />);
                        ev.stopPropagation();
                    }}><TrashIcon /></div>
                </div>
            </div>
            <div>
                <p class="text-[14px] text-ellipsis overflow-hidden">{props.project.metadata.description}</p>
            </div>
        </div>
    </div>;
};