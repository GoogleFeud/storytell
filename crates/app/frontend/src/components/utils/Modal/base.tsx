import { ParentProps } from "solid-js";


export const ModalBase = (props: ParentProps<{
    position?: "bottom" | "center"
}>) => {
    return <div class="absolute inset-0 w-screen h-screen flex justify-center items-end z-[1000] bg-[rgba(1,1,1,0.3)]" style={{
        "align-items": props.position === "bottom" ? "flex-end" : "center"
    }}>
        <div class="p-4 shadow-2xl animate-[slide-top_0.5s_cubic-bezier(0.860,_0.000,_0.070,_1.000)_both] bg-[#242424] border border-neutral-700">
            {props.children}
        </div>
    </div>;
};