import { Explorer } from "./Explorer";
import { HorizontalResize } from "./Resizables/Horizontal";
import { VerticalResize } from "./Resizables/Vertical";


export const Editor = () => {
    return <div class="h-full w-full overflow-y-hidden">
        <HorizontalResize minWLeft={600} defaultWLeft={400}>
            <HorizontalResize minWLeft={300} minWRight={300} maxWLeft={600}>
                <Explorer />
                <p>Editor</p>
            </HorizontalResize>
            <VerticalResize minHTop="200px" minHBottom="0px" defaultHTop="800px">
                <p>Live View</p>
                <p>Debug Window</p>
            </VerticalResize>
        </HorizontalResize>
    </div>;
};