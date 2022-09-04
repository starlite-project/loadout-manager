import React from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Store } from './plugins/Store';
import { error } from './plugins/Log';

export class OauthButton extends React.Component<{}, { isLoggedIn: boolean }> {
    public constructor(props: {}) {
        super(props);

        this.state = { isLoggedIn: false }
    }

    private async handleLoginClick() {
        // const code = await invoke('get_authorization_code');
        let code;

        try {
            code = await invoke('get_authorization_code');
        } catch (err) {
            await error((err as Error).message);
            return;
        }

        const store = new Store();

        console.log(await store.get('login'));

        this.setState({
            isLoggedIn: true,
        });
    }

    private async handleLogoutClick() {
        this.setState({
            isLoggedIn: false,
        });
    }

    public render(): React.ReactNode {
        // const isLoggedIn = this.state.isLoggedIn;

        // let button = null;
        // if (isLoggedIn) {
        //     button = <LogoutButton onClick={() => this.handleLogoutClick()} />;
        // } else {
        //     button = <LoginButton onClick={async () => await this.handleLoginClick()} />;
        // }

        // return (
        //     <div>{button}</div>
        // )
        return (
            <LoginButton onClick={async () => await this.handleLoginClick()} />
        );
    }
}

function LogoutButton(props: { onClick: React.MouseEventHandler<HTMLButtonElement> | undefined; }): JSX.Element {
    return (
        <button onClick={props.onClick}>Logout</button>
    )
}

function LoginButton(props: { onClick: React.MouseEventHandler<HTMLButtonElement> | undefined; }): JSX.Element {
    return (
        <button onClick={props.onClick}>Login</button>
    )
}