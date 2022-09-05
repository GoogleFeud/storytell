
export const Button = (props: {
    text: string,
    onClick?: () => void,
    primary?: boolean
}) => {
    return <div class={`rounded ${props.primary ? "bg-[#6d4c41]" : "bg-[#404040]"} text-white text-[14px] px-4 py-2 flex justify-center items-center cursor-pointer min-w-[90px]`} onClick={props.onClick}>
        {props.text}
    </div>;
};
