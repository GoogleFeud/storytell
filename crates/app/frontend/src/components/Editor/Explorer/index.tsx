import { Panel } from "../Common/Panel";
import { VerticalResize } from "../Panels/Resizables/Vertical";

export const enum Viewing {
    Files,
    Paths
}


export const Explorer = () => {
    return <div>
        <VerticalResize minHBottom={50} minHTop={50} defaultHTop={900}>
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
    </div>;
};