import { ParentProps } from "solid-js";
import { setModal } from "../../../state";


export const ModalBase = (props: ParentProps<{
    position?: "bottom" | "center",
    exitable?: boolean
}>) => {
    // eslint-disable-next-line prefer-const
    let ref: HTMLDivElement | undefined = undefined;
    return <div class="absolute inset-0 w-screen h-screen flex justify-center items-end z-[1000] bg-[rgba(1,1,1,0.3)]" ref={ref} style={{
        "align-items": props.position === "bottom" ? "flex-end" : "center"
    }} onClick={(ctx) => {
        if (props.exitable && ctx.target === ref) setModal();
    }}>
        <div class="p-4 shadow-2xl animate-[slide-top_0.5s_cubic-bezier(0.860,_0.000,_0.070,_1.000)_both] bg-[#242424] border border-neutral-700 max-w-[720px]">
            {props.children}
        </div>
    </div>;
};