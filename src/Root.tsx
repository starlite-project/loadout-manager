import { Router } from "react-router";
import { BrowserRouter } from "react-router-dom";
import App from "./App";

export function Root({ loggedIn }: Props): JSX.Element {
    return (
        <BrowserRouter>
            <App loggedIn={loggedIn} />
        </BrowserRouter>
    )
}

interface Props {
    loggedIn?: boolean;
}

export default Root;