import { createSignal, JSXElement, ParentProps } from "solid-js";
import { ArrowDownIcon } from "../../Icons/arrowDown";
import { ArrowRightIcon } from "../../Icons/arrowRight";
import { ScrollBox } from "../../utils/Scrollbar";

export const Panel = (props: ParentProps<{
    text: string,
    collapsable?: boolean,
    isCollapsed?: boolean,
    options?: JSXElement
}>) => {
    const [collapsed, setCollapsed] = createSignal(props.isCollapsed || false);

    return <ScrollBox>
        <div class="flex flex-col h-full">
            <div class="flex justify-between items-center p-1 bg-neutral-900 cursor-pointer" onClick={() => setCollapsed(!collapsed())}>
                <div class="flex items-center gap-2">
                    {props.collapsable && (collapsed() ? <ArrowRightIcon size="12px" /> : <ArrowDownIcon size="12px" />)}
                    <p class="text-[12px] text-neutral-400">{props.text.toUpperCase()}</p>
                </div>
                {props.options}
            </div>
            <div class="pl-[14px] bg-[#161616fb]">
                {!collapsed() && props.children}
            </div>
        </div>
    </ScrollBox>;
};