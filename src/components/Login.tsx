import { invoke } from '@tauri-apps/api/tauri';
import { error } from '../plugins/Log';
import { useNavigate } from 'react-router-dom';
import { TokenUtils } from '../utils';
import { useThunkDispatch } from '../store/thunk';
import { loggedIn } from '../store/account/actions';

export function Login() {
    const navigate = useNavigate();
    const dispatch = useThunkDispatch();

    return (
        <button onClick={async () => {
            try {
                const authToken = await invoke('get_authorization_code') as TokenUtils.AuthTokens;
                TokenUtils.setToken(authToken);
                dispatch(loggedIn());
            } catch (e) {
                await error(e as string);
            } finally {
                navigate("/");
            }
        }}>Login</button>
    )
}

export default Login;