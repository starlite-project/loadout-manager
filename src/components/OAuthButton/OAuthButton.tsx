import React, { useEffect, useState } from 'react';
import { canRefreshToken, deleteToken, getToken, isTokenValid, refreshToken } from './token';
import './OAuthButtons.css';

function LoginButton(props: { setLoggedIn: React.Dispatch<React.SetStateAction<boolean>> }): JSX.Element {
    return (
        <button onClick={async () => {
            await getToken();
            props.setLoggedIn(true);
        }}>Login</button>
    )
}

function LogoutButton(props: { setLoggedIn: React.Dispatch<React.SetStateAction<boolean>> }): JSX.Element {
    return (
        <button onClick={async () => {
            await deleteToken();
            props.setLoggedIn(false);
        }}>Logout</button>
    )
}

export function OAuthButton(): JSX.Element {
    const [loggedIn, setLoggedIn] = useState(false);
    useEffect(() => {
        const setLoginState = async () => {
            const validToken = await isTokenValid();
            if (!validToken) {
                const refreshableToken = await canRefreshToken();
                if (refreshableToken) {
                    await refreshToken();
                    setLoggedIn(true);
                }
            } else {
                setLoggedIn(false);
            }
        };

        setLoginState();
    }, [loggedIn]);

    return loggedIn ? (
        <LoginButton setLoggedIn={setLoggedIn} />
    ) : (
        <LogoutButton setLoggedIn={setLoggedIn} />
    );
}

export default OAuthButton;