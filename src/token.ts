import { invoke } from '@tauri-apps/api/tauri';
import React, { useEffect, useState } from 'react';
import { error, info } from './plugins/Log';

export const getToken = async () => {
    try {
        await invoke('get_authorization_code');
    } catch (e) {
        await error(e as string);
    }
}

export const refreshToken = async () => {
    try {
        await invoke('refresh_token');
    } catch (e) {
        await error(e as string);
    }
}

export const isTokenValid = async (): Promise<boolean> => await invoke('is_token_valid');

export const canRefreshToken = async (): Promise<boolean> => await invoke('is_token_refreshable');

export const deleteToken = async (): Promise<boolean> => await invoke('delete_token');

export const useToken = (): [boolean, React.Dispatch<React.SetStateAction<boolean>>] => {
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

    return [loggedIn, setLoggedIn];
}