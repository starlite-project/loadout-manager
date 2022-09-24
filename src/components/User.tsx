import { FC, useEffect, useState } from 'react';
import { GeneralUser, getUser } from '../models';

export const User: FC = () => {
    const [user, setUser] = useState<GeneralUser | undefined>();
    useEffect((): void => {
        getUser().then((response): void => setUser(response?.Response));
    }, []);

    return (
        <div>
            {JSON.stringify(user)}
        </div>
    )
}

export default User;