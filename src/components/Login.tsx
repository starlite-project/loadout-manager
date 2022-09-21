import { invoke } from '@tauri-apps/api/tauri';
import { error } from '../plugins/Log';
import { useNavigate } from 'react-router-dom';
import { token } from '../utils';

export function Login() {
    const navigate = useNavigate();

    return (
        <button onClick={async () => {
            try {
                const authToken = await invoke('get_authorization_code') as token.AuthTokens;
                token.setToken(authToken);
            } catch (e) {
                await error(e as string);
            } finally {
                navigate("/");
            }
        }}>Login</button>
    )
}

export default Login;