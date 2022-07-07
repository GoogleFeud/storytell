import { JSX } from "solid-js/jsx-runtime"


export const Text = (props: JSX.IntrinsicElements["p"]) => {
    return <p {...props} class="text-[#ffeac9] dark:text-[#1c1c1c]">
        {props.children}
    </p>
}