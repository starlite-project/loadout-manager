import { invoke } from '@tauri-apps/api/tauri';
import { error } from '../../plugins/Log';
import { useNavigate } from 'react-router-dom';
import { AuthTokens, setToken } from '../../utils/token';
import { useThunkDispatch } from '../../store/thunk';
import { loggedIn } from '../../store/account/actions';
import type { FC } from 'react';
import styles from './Login.module.scss';

export const Login: FC<{}> = () => {
    const navigate = useNavigate();
    const dispatch = useThunkDispatch();

    const onLoginClick = async (): Promise<void> => {
        try {
            const authToken = await invoke('get_authorization_code') as AuthTokens;
            setToken(authToken);
            dispatch(loggedIn());
        } catch (e) {
            await error(e as string);
        } finally {
            navigate("/");
        }
    }

    return (
        <div className={styles.billboard}>
            <button onClick={onLoginClick}>Login</button>
        </div>
    )
}

export default Login;