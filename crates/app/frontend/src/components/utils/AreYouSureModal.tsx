import { setModal } from "../../state";


export const AreYouSureModal = (props: { text?: string, onYes?: () => void, onNo?: () => void }) => {
    return <div class="w-[25%] bg-white p-4 shadow-2xl absolute top-[40%] left-[45%] z-[1000] flex flex-col gap-2 rounded animate-[slide-top_0.5s_cubic-bezier(0.860,_0.000,_0.070,_1.000)_both]">
        <h3 class="text-[22px] font-semibold">Are you sure?</h3>
        <p>{props.text}</p>
        <div class="flex gap-3 justify-evenly">
            <div class="bg-[#7D9D9C] rounded text-[18px] p-1 cursor-pointer" onClick={() => {
                props.onYes?.();
                setModal();
            }}>
                <p>Yes</p>
            </div>
            <div class="bg-[#7D9D9C] rounded text-[18px] p-1 cursor-pointer" onClick={() => {
                props.onNo?.();
                setModal();
            }}>
                <p>Cancel</p>
            </div>
        </div>
    </div>;
};