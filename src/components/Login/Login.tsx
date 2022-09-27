import { invoke } from '@tauri-apps/api/tauri';
import { error } from '../../plugins/Log';
import { useNavigate } from 'react-router-dom';
import { AuthTokens, setToken } from '../../utils/token';
import { useThunkDispatch } from '../../store/thunk';
import { loggedIn } from '../../store/account/actions';
import type { FunctionComponent } from 'react';
import { Billboard, AuthButton } from './styles';
import { t } from '../../utils';
import type React from 'react';

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

    return <Billboard>
        <AuthButton
            rel="noopener noreferrer"
            onClick={onLoginClick}>{t('Views.Login.Auth')}</AuthButton>
    </Billboard>
}

export default Login;