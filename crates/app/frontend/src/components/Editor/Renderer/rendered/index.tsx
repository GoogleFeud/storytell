import { ASTBlock, ASTBlockKind } from "@types";
import { Navigatable } from "./navigatable";
import { renderText } from "./text";


export const renderBlock = (block: ASTBlock) => {
    switch(block.kind) {
    case ASTBlockKind.Paragraph: {
        const text = renderText(block);
        if (!text) return;
        return <Navigatable range={block.range}>{text}</Navigatable>;
    }
    default:
        return <Navigatable range={block.range}>Something</Navigatable>;
    }
};