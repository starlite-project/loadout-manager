import { invoke } from '@tauri-apps/api/tauri';
import { error } from '../../plugins/Log';
import { useNavigate } from 'react-router-dom';
import { AuthTokens, setToken } from '../../utils/token';
import { useThunkDispatch } from '../../store/thunk';
import { loggedIn } from '../../store/account/actions';
import type { FunctionComponent } from 'react';
import styles from './Login.module.scss';
import { t } from '../../utils';
import type React from 'react';

// export const Login: FunctionComponent = () => {
//     const navigate = useNavigate();
//     const dispatch = useThunkDispatch();

//     const onLoginClick = async (): Promise<void> => {
//         try {
//             const authToken = await invoke('get_authorization_code') as AuthTokens;
//             setToken(authToken);
//             dispatch(loggedIn());
//         } catch (e) {
//             await error(e as string);
//         } finally {
//             navigate("/");
//         }
//     }

//     return (
//         <div className={styles.billboard}>
//             <button onClick={onLoginClick}>{t('Views.Login.Auth')}</button>
//         </div>
//     )
// }

export const Login: FunctionComponent = () => {
    const navigate = useNavigate();
    const dispatch = useThunkDispatch();

    const onLoginClick = async (e: React.MouseEvent) => {
        e.preventDefault();
        try {
            const authToken = await invoke('get_authorization_code') as AuthTokens;
            setToken(authToken);
            dispatch(loggedIn());
        } catch (e) {
            await error(e as string);
        } finally {
            navigate('/');
        }
    };

    return <div className={styles.billboard}>
        <a rel="noopener noreferrer"
            onClick={onLoginClick}
            className={styles.auth}
        >
            {t('Views.Login.Auth')}
        </a>
    </div>
}

export default Login;