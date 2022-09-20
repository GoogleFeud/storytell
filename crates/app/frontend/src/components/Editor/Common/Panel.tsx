import { children, createSignal, JSXElement, ParentProps } from "solid-js";
import { ArrowDownIcon } from "@icons/arrowDown";
import { ArrowRightIcon } from "@icons/arrowRight";
import { ScrollBox } from "@utils/Scrollbar";

export const Panel = (props: ParentProps<{
    text: string,
    collapsable?: boolean,
    isCollapsed?: boolean,
    options?: JSXElement
}>) => {
    const [collapsed, setCollapsed] = createSignal(props.isCollapsed || false);
    const realChildren = children(() => props.children);

    return <ScrollBox>
        <div class="flex flex-col h-full border-y border-neutral-800">
            <div class="flex justify-between items-center p-1 cursor-pointer" onClick={() => setCollapsed(!collapsed())}>
                <div class="flex items-center gap-2 select-none">
                    {props.collapsable && (collapsed() ? <ArrowRightIcon size="12px" /> : <ArrowDownIcon size="12px" />)}
                    <p class="text-[12px] text-neutral-400">{props.text.toUpperCase()}</p>
                </div>
                {props.options}
            </div>
            {!collapsed() && <div class="pl-[14px] h-full">
                {realChildren()}
            </div>}
        </div>
    </ScrollBox>;
};