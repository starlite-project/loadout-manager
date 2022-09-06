import { invoke } from '@tauri-apps/api/tauri';
import { useEffect, useState } from 'react';
import { BungieResponse } from '.';

const getUser = async (): Promise<BungieResponse<GeneralUser | null>> => {
    const raw = await invoke('get_current_user');

    return raw as BungieResponse<GeneralUser | null>;
}

interface GeneralUser {
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

export default function User() {
    const [user, setUser] = useState<GeneralUser | null>(null);
    useEffect(() => {
        getUser().then((response) => setUser(response.Response));
    }, []);

    return (
        <div>
            {JSON.stringify(user)}
        </div>
    )
}