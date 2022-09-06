import { ThreeDotsIcon } from "../../Icons/threeDots";
import { Panel } from "../Common/Panel";
import { VerticalResize } from "../Panels/Resizables/Vertical";

export const Explorer = () => {
    return <div class="flex flex-col">
        <div class="flex justify-between items-center border-t border-neutral-800 p-1.5 px-3">
            <p class="text-[12px]">EXPLORER</p>
            <div class="text-neutral-400">
                <ThreeDotsIcon />
            </div>
        </div>
        <div class="h-full">
            <VerticalResize minHBottom="50px" minHTop="50px" defaultHTop={(80 / 100 * window.screen.height) + "px"}>
                <Panel text="Files" collapsable>
                    <div class="pt-2">
                        <p>File 1...</p>
                        <p>File 2...</p>
                        <p>File 3...</p>
                    </div>
                </Panel>
                <Panel text="Paths" collapsable isCollapsed>
                    <div class="pt-2">
                        <p>Path 1...</p>
                        <p>Path 2...</p>
                        <p>Path 3...</p>
                    </div>
                </Panel>
            </VerticalResize>
        </div>
    </div>;
};