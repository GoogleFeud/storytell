import { setEditor } from "@state/editor";
import * as monaco from "monaco-editor/esm/vs/editor/editor.api";
import { onMount } from "solid-js";
import "./worker";

export const MonacoEditor = () => {
    let editorRef: HTMLDivElement|undefined;

    onMount(() => {
        setEditor(monaco.editor.create(editorRef as HTMLDivElement, {
            language: "markdown",
            automaticLayout: true,
            theme: "vs-dark"
        }));
    });

    return <div class="h-full w-full">
        <div class="h-full w-full" ref={editorRef}></div>
    </div>;
};