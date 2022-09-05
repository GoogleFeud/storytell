import { setModal } from "../../../state";
import { XIcon } from "../../Icons/x";
import { ModalBase } from "./base";


export const MessageModal = (props: { text: string }) => {
    return <ModalBase position="bottom">
        <div class="flex justify-between items-center gap-12">
            <p>{props.text}</p>
            <div class="cursor-pointer" onClick={() => setModal(undefined)}>
                <XIcon></XIcon>
            </div>
        </div>
    </ModalBase>;
};