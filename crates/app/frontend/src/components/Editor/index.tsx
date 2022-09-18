import { Explorer } from "./Explorer";
import { ResizablePane } from "./Common/Resizables";
import { TextEditor } from "./TextEditor";

export const Editor = () => {
    return <div class="h-full w-full overflow-y-hidden">
        <ResizablePane sizes={[20, 60, 20]}>
            <Explorer />
            <TextEditor />
            <ResizablePane vertical sizes={[70, 30]}>
                <p>Live View</p>
                <p>Debug Window</p>
            </ResizablePane>
        </ResizablePane>
    </div>;
};