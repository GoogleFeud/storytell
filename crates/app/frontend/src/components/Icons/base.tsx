import { JSXElement, ParentProps } from "solid-js";


export const CustomIcon = (props: ParentProps<{
    size?: string,
    onClick?: (ev: MouseEvent) => void
}>) => {
    if (props.onClick) return <div class="cursor-pointer" onClick={props.onClick}>
        <svg xmlns="http://www.w3.org/2000/svg" width={props.size || "16"} height={props.size || "16"} fill="currentColor" viewBox="0 0 16 16">
            {props.children}
        </svg>
    </div>;
    else return <svg xmlns="http://www.w3.org/2000/svg" width={props.size || "16"} height={props.size || "16"} fill="currentColor" viewBox="0 0 16 16">
        {props.children}
    </svg>;
};

export const makeCustomIcon = (children: () => JSXElement[]) => {
    return (props: { size?: string, onClick?: (ev: MouseEvent) => void}) => {
        return <CustomIcon {...props}>
            {children()}
        </CustomIcon>;
    };
};