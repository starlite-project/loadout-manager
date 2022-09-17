import { BungieResponse } from '.';
import { invoke } from '@tauri-apps/api/tauri';

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

export const getUser = async (): Promise<BungieResponse<GeneralUser | null>> => {
    const raw = await invoke('get_current_user');

    return raw as BungieResponse<GeneralUser | null>;
}