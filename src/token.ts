import { invoke } from '@tauri-apps/api/tauri';
import { error } from './plugins/Log';

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