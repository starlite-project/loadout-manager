import { invoke } from '@tauri-apps/api/tauri';
import { useEffect, useState } from 'react';
import { GeneralUser, getUser } from '../models';

export function User() {
    const [user, setUser] = useState<GeneralUser | null>(null);
    useEffect(() => {
        getUser().then((response) => {
            console.log(response);
            setUser(response?.Response ?? null);
        });
    }, []);

    return (
        <div>
            {JSON.stringify(user)}
        </div>
    )
}

export default User;