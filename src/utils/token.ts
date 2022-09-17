import { error } from '../plugins/Log';
import { invoke } from '@tauri-apps/api/tauri';
import { useState, useEffect } from 'react';

export const isLoggedIn = (): boolean => {
    const [loggedIn, setLoggedIn] = useState(false);
    useEffect(() => {
        const setLoggedInState = async () => {
            try {
                setLoggedIn(await invoke('logged_in'));
            } catch (e) {
                await error(e as string);
                setLoggedIn(false);
            }
        }

        setLoggedInState();
    }, [loggedIn]);

    return loggedIn

    // try {
    //     return await invoke('logged_in');
    // } catch (e) {
    //     await error(e as string);
    // }

    // return false;
}