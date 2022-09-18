/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { Accessor, createSignal, Setter } from "solid-js";
import { JSX } from "solid-js/jsx-runtime";

export const Input = (props: JSX.InputHTMLAttributes<HTMLInputElement> & {
    onExit?: (value: string, error: Accessor<string | undefined>, setError: Setter<string | undefined>) => void,
    validator?: (value: string) => string | undefined
}) => {
    const [error, setError] = createSignal<string>();
    let done = false;
    // eslint-disable-next-line prefer-const
    let input_ref: HTMLInputElement|undefined = undefined;
    return <>
        <input {...props} ref={input_ref} class={error() ? `${props.class || ""} border border-red-600` : props.class} onKeyDown={(ev) => {
            if (ev.key === "Enter" && !error()) {
                done = true;
                props.onExit?.((ev.target as HTMLInputElement).value, error, setError);
            }
        }} onBlur={(ev) => {
            if (done || error()) return;
            props.onExit?.((ev.target as HTMLInputElement).value, error, setError);
        }} onInput={((input) => {
            if (!props.validator) return;
            const validated = props.validator!((input.target as HTMLInputElement).value);
            if (validated) setError(validated);
            else setError();
        })} />
        {error() && <div class="absolute bg-red-600 p-1" style={{
            top: `${input_ref!.getBoundingClientRect().top + input_ref!.getBoundingClientRect().height}px`,
            left: `${input_ref!.getBoundingClientRect().left}px`,
            width: `${input_ref!.getBoundingClientRect().width}px`
        }}>
            <p class="text-white text-[12px]">{error()}</p>
        </div>}
    </>;
};