import "./style.css";
import { Match, render, Switch } from "solid-js/web";
import { TitleScreen } from "./components/TitleScreen";
import { Pages, state } from "./state";
import { Editor } from "./components/Editor";

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
