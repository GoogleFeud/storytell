import { ASTBlock, ASTBlockKind } from "@types";
import { Navigatable } from "./navigatable";
import { RenderText } from "./text";


export const RenderBlock = (props: {
    block: ASTBlock
}) => {
    switch(props.block.kind) {
    case ASTBlockKind.Paragraph:
        return <RenderText item={props.block} navigatable></RenderText>;
    default:
        return <Navigatable range={props.block.range}>Something</Navigatable>;
    }
};