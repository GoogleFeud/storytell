import { ASTInlineText, ASTInlineTextKind, ASTParagraph, ASTText, ASTTextPart } from "@types";
import { For, JSXElement } from "solid-js";
import { Navigatable } from "./navigatable";

export const renderInline = (inline: ASTInlineText) : JSXElement => {
    switch(inline.kind) {
    case ASTInlineTextKind.Bold:
        return <b><RenderText item={inline.text as ASTText} /></b>;
    case ASTInlineTextKind.Code:
        return <code><RenderText item={inline.text as ASTText} /></code>;
    case ASTInlineTextKind.Italics:
        return <i><RenderText item={inline.text as ASTText} /></i>;
    case ASTInlineTextKind.Underline:
        return <u><RenderText item={inline.text as ASTText} /></u>;
    case ASTInlineTextKind.JavaScript:
        return "<somejs>";
    case ASTInlineTextKind.Join:
        return <></>;
    }
};

export const renderTextPart = (text: ASTTextPart) : JSXElement => {
    return <span>{text.before}{text.text && renderInline(text.text)}</span>;
};

export const RenderText = (props: {
    item: ASTText | ASTParagraph,
    navigatable?: boolean
}) => {
    if (props.navigatable) return <Navigatable range={props.item.range}>
        <For each={props.item.parts || []}>{(part) => {
            return renderTextPart(part);
        }}</For>{props.item.tail}
    </Navigatable>;
    else return <span>
        <For each={props.item.parts || []}>{(part) => {
            return renderTextPart(part);
        }}</For>{props.item.tail}
    </span>;
};