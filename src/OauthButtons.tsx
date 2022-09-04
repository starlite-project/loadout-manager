import React from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Store } from './plugins/Store';
import { error } from './plugins/Log';

// export class OauthButton extends React.Component<{}, { isLoggedIn: boolean }> {
//     public constructor(props: {}) {
//         super(props);

//         this.state = { isLoggedIn: false }
//     }

//     private async handleLoginClick() {
//         try {
//             await invoke('get_authorization_code');
//         } catch (err) {
//             await error((err as Error).message);
//             return;
//         }
//     }

//     public render(): React.ReactNode {
//         return <button onClick={async () => await this.handleLoginClick()}>Login</button>
//     }
// }

// function LogoutButton(props: { onClick: React.MouseEventHandler<HTMLButtonElement> | undefined; }): JSX.Element {
//     return (
//         <button onClick={props.onClick}>Logout</button>
//     )
// }

// function LoginButton(props: { onClick: React.MouseEventHandler<HTMLButtonElement> | undefined; }): JSX.Element {
//     return (
//         <button onClick={props.onClick}>Login</button>
//     )
// }

export default null;