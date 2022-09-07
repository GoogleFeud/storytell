import "./style.css";
import { Match, render, Switch } from "solid-js/web";
import { TitleScreen } from "./components/TitleScreen";
import { state } from "./state";
import { Editor } from "./components/Editor";
import { Pages } from "./types";

const App = () => {
    return <div class="h-full">
        {state.modal}
        <Switch>
            <Match when={state.currentPage === Pages.TitleScreen}>
                <TitleScreen />
            </Match>
            <Match when={state.currentPage === Pages.Editor}>
                <Editor />
            </Match>
        </Switch>
    </div>;
};

render(() => <App />, document.getElementById("root") as HTMLElement);
