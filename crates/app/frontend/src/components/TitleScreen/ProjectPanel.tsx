import { deleteProject } from "../../state";
import { Project } from "../../types";
import { DownloadIcon } from "../Icons/download";
import { GearIcon } from "../Icons/gear";
import { TrashIcon } from "../Icons/trash";


export const ProjectPanel = (props: {
    project: Project
}) => {
    return <div class="w-[50vw] h-[89px] bg-[#E4DCCF] drop-shadow-lg rounded-lg flex flex-col gap-3 cursor-pointer px-[18px] py-[12px] hover:scale-[1.008] transition">
        <div class="flex justify-between items-center">
            <p class="text-[20px]">{props.project.metadata.name}</p>
            <div class="flex gap-4">
                <GearIcon />
                <DownloadIcon />
                <div onClick={() => deleteProject(props.project.metadata.name)}><TrashIcon /></div>
            </div>
        </div>
        <div>
            <p class="text-[14px]">{props.project.metadata.description}</p>
        </div>
    </div>;
};