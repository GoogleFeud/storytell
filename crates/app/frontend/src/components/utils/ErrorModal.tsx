import { setModal } from "../../state";
import { XIcon } from "../Icons/x";


export const ErrorModal = (props: { msg: string }) => {
    return <div class="w-[50%] p-4 shadow-2xl absolute top-[90%] left-[25%] rounded flex justify-between items-center animate-[slide-top_0.5s_cubic-bezier(0.860,_0.000,_0.070,_1.000)_both]">
        <p>{props.msg}</p>
        <div class="cursor-pointer" onClick={() => setModal(undefined)}>
            <XIcon></XIcon>
        </div>
    </div>;
};