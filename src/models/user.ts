import { BungieResponse, getActiveToken } from '.';
import { invoke } from '@tauri-apps/api/tauri';
import { TokenUtils } from '../utils';
import { error } from '../plugins/Log';

export interface GeneralUser {
    membershipId: string;
    uniqueName: string;
    normalizedName: string;
    displayName: string;
    profilePicture: number;
    profileTheme: number;
    userTitle: number;
    successMessageFlags: string;
    isDeleted: boolean;
    about: string;
    psnDisplayName: string;
    xboxDisplayName: string;
    fbDisplayName: string;
    showActivity: boolean | null;
    locale: string;
}

export const getUser = async (): Promise<BungieResponse<GeneralUser> | null> => {
    const token = await getActiveToken();

    if (!token) {
        return null;
    }

    let raw = null;
    try {
        raw = await invoke('get_current_user', { token })
    } catch (e) {
        await error((e as Error).message);
    }

    return raw as BungieResponse<GeneralUser> | null;
}