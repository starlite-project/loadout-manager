import { invoke } from '@tauri-apps/api';
import { Store } from 'tauri-plugin-store-api';
import { AuthorizationToken } from './models';

export const getToken = async () => {
    const store = new Store('.token');

    const currentToken = await store.get('login') as AuthorizationToken | null;

    if (!currentToken || currentToken.expiresIn < Date.now()) {
        const newToken = await invoke('get_authorization_code') as AuthorizationToken;

        await store.set('login', newToken);
    }

    const token = await store.get('login') as AuthorizationToken;

    console.log(token);

    return token;
}