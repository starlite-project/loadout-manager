import { Router } from "react-router";
import { BrowserRouter } from "react-router-dom";
import App from "./App";

export function Root(): JSX.Element {
    return (
        <BrowserRouter>
            <App />
        </BrowserRouter>
    )
}

interface Props {
    loggedIn?: boolean;
}

export default Root;