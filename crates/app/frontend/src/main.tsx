import "./style.css";
import { render } from "solid-js/web";
import { TitleScreen } from "./components/TitleScreen";
import { state } from "./state";

const App = () => {
    return <div>
        {state.modal}
        <TitleScreen />
    </div>;
};

render(() => <App />, document.getElementById("root")!);
