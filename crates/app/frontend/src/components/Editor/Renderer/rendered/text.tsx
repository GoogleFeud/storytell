import { setJoinNext } from "@state/renderer";
import { ASTInlineText, ASTInlineTextKind, ASTParagraph, ASTText, ASTTextPart } from "@types";
import { JSXElement } from "solid-js";

export const renderInline = (inline: ASTInlineText) : JSXElement => {
    switch(inline.kind) {
    case ASTInlineTextKind.Bold:
        return <b>{renderText(inline.text as ASTText)}</b>;
    case ASTInlineTextKind.Code:
        return <code>{renderText(inline.text as ASTText)}</code>;
    case ASTInlineTextKind.Italics:
        return <i>{renderText(inline.text as ASTText)}</i>;
    case ASTInlineTextKind.Underline:
        return <u>{renderText(inline.text as ASTText)}</u>;
    case ASTInlineTextKind.JavaScript:
        return "<somejs>";
    case ASTInlineTextKind.Join:
        setJoinNext(true);
        return undefined;
    }
};

export const renderTextPart = (text: ASTTextPart) : JSXElement => {
    const rendered = text.text && renderInline(text.text);
    if (!text.before && !rendered) return undefined;
    return <span>{text.before}{rendered}</span>;
};

export const renderText = (item: ASTText | ASTParagraph) => {
    const textParts = item.parts?.map(p => renderTextPart(p)).filter(p => p);
    if (!textParts.length && !item.tail) return;
    return <span>
        {textParts}
        <span>{item.tail}</span>
    </span>;
};