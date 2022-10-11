import { invoke } from '@tauri-apps/api/tauri';
import type React from 'react';
import { useNavigate } from 'react-router';
import { error } from '../plugins/Log';
import { loggedIn } from '../store/account/actions';
import { useThunkDispatch } from '../store/thunk';
import { t } from '../utils';
import { AuthTokens, setToken } from '../utils/token';

export const Login: React.FunctionComponent = () => {
    const navigate = useNavigate();
    const dispatch = useThunkDispatch();

    const onLoginClick = async (e: React.MouseEvent): Promise<void> => {
        e.preventDefault();
        try {
            const authToken = (await invoke('get_authorization_code')) as AuthTokens;
            setToken(authToken);
            dispatch(loggedIn());
        } catch (e) {
            await error(e as string);
        } finally {
            navigate('/');
        }
    }

    return (
        <div>
            <a rel='noopener noreferrer' onClick={onLoginClick}>
                {t('Views.Login.Auth')}
            </a>
        </div>
    )
}

export default Login;