
export const Button = (props: {
    text: string,
    onClick?: () => void,
    primary?: boolean
}) => {
    return <div class={`rounded ${props.primary ? "bg-[#6d4c41] hover:bg-[#8f6657]" : "bg-[#404040] hover:bg-[#595959]"} text-white text-[14px] px-4 py-2 flex justify-center items-center cursor-pointer transition-all min-w-[90px]`} onClick={props.onClick}>
        {props.text}
    </div>;
};
