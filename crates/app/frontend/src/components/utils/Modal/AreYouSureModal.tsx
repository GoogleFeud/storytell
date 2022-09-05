import { setModal } from "../../../state";
import { Button } from "../../Input/Button";
import { ModalBase } from "./base";


export const AreYouSureModal = (props: { text?: string, onYes?: () => void, onNo?: () => void }) => {
    return <ModalBase position="center">
        <div class="flex flex-col gap-4">
            <h3 class="text-[18px] font-semibold">Are you sure?</h3>
            <p class="text-[15px]">{props.text}</p>
            <div class="flex gap-3 justify-evenly">
                <Button text="Yes" onClick={() => {
                    props.onYes?.();
                    setModal();
                }} />
                <Button text="No" onClick={() => {
                    props.onNo?.();
                    setModal();
                }} />
            </div>
        </div>
    </ModalBase>;
};