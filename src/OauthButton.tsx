import React from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Store } from 'tauri-plugin-store-api';

export class OauthButton extends React.Component<{}, { isLoggedIn: boolean }> {
    public constructor(props: {}) {
        super(props);

        this.state = { isLoggedIn: false }
    }

    private async handleLoginClick() {
        const code = await invoke('get_authorization_code');

        const store = new Store('.token');

        console.log(await store.get('login'));

        await store.set('login', code);

        console.log(store);

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