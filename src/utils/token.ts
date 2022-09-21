import { error } from '../plugins/Log';
import { invoke } from '@tauri-apps/api/tauri';
import { useState, useEffect } from 'react';

// export const useLoggedIn = (): boolean => {
//     const [loggedIn, setLoggedIn] = useState(true);
//     useEffect(() => {
//         (async () => {
//             if (!await isTokenValid()) {
//                 if (await canRefreshToken()) {
//                     await refreshToken();
//                     setLoggedIn(true);
//                 } else {
//                     setLoggedIn(false);
//                 }
//             } else {
//                 setLoggedIn(true);
//             }
//         })();
//     }, [loggedIn]);

//     return loggedIn;
// }

export const isLoggedIn = async (): Promise<boolean> => {
    if (!await isTokenValid()) {
        if (await canRefreshToken()) {
            return refreshToken().then((): boolean => true);
        } else {
            return false;
        }
    } else {
        return true;
    }
}

export const refreshToken = async (): Promise<void> => {
    try {
        await invoke('refresh_token');
    } catch (e) {
        await error((e as Error).message);
    }
}

export const getToken = async (): Promise<void> => {
    try {
        await invoke('get_authorization_code');
    } catch (e) {
        await error((e as Error).message);
    }
}

export const isTokenValid = async (): Promise<boolean> => await invoke('is_token_valid');

export const canRefreshToken = async (): Promise<boolean> => await invoke('is_token_refreshable');

export const deleteToken = async (): Promise<boolean> => await invoke('plugin:storage|delete', { key: 'auth_data' });