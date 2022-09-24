import { FC, useEffect, useState } from 'react';
import { GeneralUser, getUser } from '../models';

export const User: FC = () => {
    const [user, setUser] = useState<GeneralUser | null>(null);
    useEffect((): void => {
        getUser().then((response): void => setUser(response?.Response ?? null));
    }, []);

    return (
        <div>
            {JSON.stringify(user)}
        </div>
    )
}

export default User;