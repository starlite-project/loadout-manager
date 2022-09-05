import React from 'react';
import { deleteToken, getToken } from './token';

export function LoginButton(props: { setLoggedIn: React.Dispatch<React.SetStateAction<boolean>> }): JSX.Element {
    return (
        <button onClick={async () => {
            await getToken();
            props.setLoggedIn(true);
        }}>Login</button>
    )
}

export function LogoutButton(props: { setLoggedIn: React.Dispatch<React.SetStateAction<boolean>> }): JSX.Element {
    return (
        <button onClick={async () => {
            await deleteToken();
            props.setLoggedIn(false);
        }}>Logout</button>
    )
}