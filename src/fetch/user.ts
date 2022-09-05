import { invoke } from '@tauri-apps/api/tauri';
import { useEffect, useState } from 'react';

export const getUser = async (): Promise<GeneralUser | null> => { 
    const raw = await invoke('get_current_user');

    return raw as GeneralUser | null;
}

export const useUser = () => {
    const [user, setUser] = useState(null);
    useEffect(() => {
        
    })
}

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