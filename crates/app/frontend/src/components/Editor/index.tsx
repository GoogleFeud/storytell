import { HorizontalResize } from "./Panels/Resizables/Horizontal";
import { VerticalResize } from "./Panels/Resizables/Vertical";


export const Editor = () => {
    return <div class="h-full w-full">
        <HorizontalResize minWLeft={600} defaultWLeft={400}>
            <HorizontalResize minWLeft={300} minWRight={300}>
                <p>File Manager</p>
                <p>Editor</p>
            </HorizontalResize>
            <VerticalResize minHTop={200} minHBottom={200} defaultHTop={800}>
                <p>Live View</p>
                <p>Debug Window</p>
            </VerticalResize>
        </HorizontalResize>
    </div>;
};