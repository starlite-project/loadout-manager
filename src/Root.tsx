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

export default Root;