import { JSXElement } from "solid-js";


export const TextField = (props: {
    onChange?: (content: string) => void,
    error?: string,
    value?: string,
    icon?: JSXElement,
    placeholder?: string
}) => {
    return <div class="flex flex-col gap-2">
        <div class="text-[13px] p-2 border rounded border-neutral-700 outline-1 flex items-center gap-3 focus-within:bg-[#282626] hover:bg-[#282626] focus-within:border-neutral-500 transition-all" classList={{
            "border-red-500": !!props.error
        }}>
            {props.icon}
            <input type="text" value={props.value || ""} class="bg-transparent outline-0 basis-full" placeholder={props.placeholder} onInput={(ctx) => props.onChange?.((ctx.target as HTMLInputElement).value)}></input>
        </div>
        {props.error && <p class="text-[13px] text-red-500">{props.error}</p>}
    </div>;
};

export const TextFieldGroup = (props: {
    title: string,
    error?: string,
    value?: string,
    onChange?: (content: string) => void,
    icon?: JSXElement,
    placeholder?: string
}) => {
    return <div class="flex flex-col gap-2">
        <p class="text-[14px]">{props.title}</p>
        <TextField {...props} />
    </div>;
};