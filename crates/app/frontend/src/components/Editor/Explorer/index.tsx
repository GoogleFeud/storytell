import { state } from "@state/index";
import { ThreeDotsIcon } from "@icons/threeDots";
import { Panel } from "../Common/Panel";
import { ResizablePane } from "../Common/Resizables";
import { FileManager } from "./Files";

export const Explorer = () => {
    return <div class="h-full">
        <div class="flex justify-between items-center border-t border-neutral-800 p-1.5 px-3">
            <p class="text-[12px]">EXPLORER</p>
            <div class="text-neutral-400">
                <ThreeDotsIcon />
            </div>
        </div>
        <div class="h-full">
            <ResizablePane vertical sizes={[70, 30]}>
                <FileManager files={state.fileExplorer.global.map(id => state.fileExplorer.blobs[id])} />
                <Panel text="Paths" collapsable isCollapsed>
                    <div class="pt-2">
                        <p>Path 1...</p>
                        <p>Path 2...</p>
                        <p>Path 3...</p>
                    </div>
                </Panel>
            </ResizablePane>
        </div>
    </div>;
};