import React from 'react';
import { invoke } from '@tauri-apps/api/tauri';

export class OauthButton extends React.Component<{ alreadyLoggedIn?: boolean }, { isLoggedIn: boolean }> {
    public constructor(props: { alreadyLoggedIn: boolean }) {
        super(props);

        // this.setState({
        //     isLoggedIn: props.alreadyLoggedIn,
        // });
        this.state = { isLoggedIn: !!props.alreadyLoggedIn }
    }

    private async handleLoginClick() {
        await invoke('begin_oauth');

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