
export const Button = (props: {
    onClick?: () => void,
    text: string,
    px?: s
}) => {
    return <div class="rounded bg-[#7D9D9C] shadow-md text-[20px] text-white px-[26px] py-[11px] h-[46px] flex justify-center items-center cursor-pointer hover:scale-[1.008] transition" onClick={props.onClick}>
        <p>{props.text}</p>
    </div>;
};