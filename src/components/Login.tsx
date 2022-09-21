import { invoke } from '@tauri-apps/api/tauri';
import { error } from '../plugins/Log';
import { useNavigate } from 'react-router-dom';

export function Login() {
    const navigate = useNavigate();

    return (
        <button onClick={async () => {
            try {
                await invoke('get_authorization_code');
            } catch (e) {
                await error(e as string);
            } finally {
                navigate("/");
            }
        }}>Login</button>
    )
}

export default Login;