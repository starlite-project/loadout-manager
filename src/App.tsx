import React, { useEffect, useState } from 'react';
import { BrowserRouter, Route, Routes } from 'react-router-dom';
import './App.css'
import { LoginButton, LogoutButton } from './OauthButtons';
import { canRefreshToken, isTokenValid, refreshToken } from './token';

export const App = () => {
    // const [isLoggedIn, setIsLoggedIn] = useState(false);

    // return (
    //     <button onClick={async () => await getToken()}>Login</button>
    // );
    const [loggedIn, setLoggedIn] = useState(false);
    useEffect(() => {
        const setLoginState = async () => {
            const validToken = await isTokenValid();
            if (!validToken) {
                const isRefreshable = await canRefreshToken();
                if (isRefreshable) {
                    await refreshToken();
                    setLoggedIn(true);
                }
            } else {
                setLoggedIn(true);
            }
        };

        setLoginState();
    }, [loggedIn]);

    return loggedIn ? (
        // <LogoutButton setLoggedIn={setLoggedIn} />
        <div>
            <LogoutButton setLoggedIn={setLoggedIn} />
        </div>
    ) : (
        <LoginButton setLoggedIn={setLoggedIn} />
    )
}

export default App;