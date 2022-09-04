import { deleteProject, setModal } from "../../../../state";
import { Project } from "../../../../types";
import { DownloadIcon } from "../../../Icons/download";
import { EditIcon } from "../../../Icons/edit";
import { TrashIcon } from "../../../Icons/trash";
import { AreYouSureModal } from "../../../utils/AreYouSureModal";


export const ProjectPanel = (props: {
    project: Project
}) => {
    return <div class="hover:scale-[1.008] transition">
        <div class="w-[50vw] h-[89px] bg-[#E4DCCF] drop-shadow-lg rounded-lg flex flex-col gap-3 cursor-pointer px-[18px] py-[12px] animate-[scale-in-center_0.2s_cubic-bezier(0.250,_0.460,_0.450,_0.940)_both]">
            <div class="flex justify-between items-center">
                <p class="text-[20px]">{props.project.metadata.name}</p>
                <div class="flex gap-4">
                    <EditIcon />
                    <DownloadIcon />
                    <div onClick={() => {
                        setModal(<AreYouSureModal text={`Do you want to delete ${props.project.metadata.name}`} onYes={() => deleteProject(props.project.metadata.name)} />);
                    }}><TrashIcon /></div>
                </div>
            </div>
            <div>
                <p class="text-[14px]">{props.project.metadata.description}</p>
            </div>
        </div>
    </div>;
};