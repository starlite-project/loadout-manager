import { BungieResponse } from '.';
import { invoke } from '@tauri-apps/api/tauri';
import { TokenUtils } from '../utils';

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
    const token = TokenUtils.getToken();

    if (!token) {
        return null;
    }

    const raw = await invoke('get_current_user', { token: token.accessToken.value, membershipId: token.bungieMembershipId });

    return raw as BungieResponse<GeneralUser>;
}