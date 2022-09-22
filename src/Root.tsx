import { BrowserRouter } from "react-router-dom";
import App from "./App";
import { Provider } from 'react-redux';
import store from './store';

export function Root(): JSX.Element {
    return (
        <BrowserRouter>
            <Provider store={store}>
                <App />
            </Provider>
        </BrowserRouter>
    )
}

interface Props {
    loggedIn?: boolean;
}

export default Root;