import { JSX } from "solid-js/jsx-runtime";

export const Input = (props: JSX.InputHTMLAttributes<HTMLInputElement> & { onExit?: (value: string) => void }) => {
    return <input {...props} onKeyDown={(ev) => {
        if (ev.key === "Enter") props.onExit?.((ev.target as HTMLInputElement).value);
    }} onBlur={(ev) => props.onExit?.((ev.target as HTMLInputElement).value)} />;
};