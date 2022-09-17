import { invoke } from '@tauri-apps/api/tauri';
import { error } from '../plugins/Log';

export function Login() {
    return (
        <button onClick={async () => {
            try {
                await invoke('refresh_token');
            } catch (e) {
                await error(e as string);
            }
        }}>Login</button>
    )
}

export default Login;