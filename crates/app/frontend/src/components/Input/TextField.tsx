import { JSXElement } from "solid-js";


export const TextField = (props: {
    onChange?: (content: string) => void,
    icon?: JSXElement,
    placeholder?: string
}) => {
    return <div class="text-[13px] p-2 border rounded border-neutral-700 outline-1 flex items-center gap-3 focus-within:bg-[#282626] focus-within:border-neutral-500 transition-all">
        {props.icon}
        <input type="text" class="bg-transparent outline-0" placeholder={props.placeholder} onChange={(ctx) => props.onChange?.((ctx.target as HTMLInputElement).value)}></input>
    </div>;
};