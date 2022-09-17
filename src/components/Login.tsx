import { invoke } from '@tauri-apps/api/tauri';
import { error } from '../plugins/Log';

export function Login() {
    return (
        <button onClick={async () => {
            try {
                await invoke('get_authorization_code');
            } catch (e) {
                await error(e as string);
            }
        }}>Login</button>
    )
}

export default Login;