import React from 'react';
import { createRoot } from 'react-dom/client';

function Welcome(props: { name: string }) {
    return <h1>Hello, {props.name}</h1>
}

class Toggle extends React.Component {
    state: Readonly<{ isToggledOn: boolean }>;

    public constructor(props: {}) {
        super(props);
        this.state = { isToggledOn: true };

        this.handleClick = this.handleClick.bind(this);
    }

    private handleClick(): void {
        this.setState((prevState: { isToggledOn: boolean }) => ({
            isToggledOn: !prevState.isToggledOn
        }));
    }

    public render(): React.ReactNode {
        return (
            <button onClick={this.handleClick}>
                {this.state.isToggledOn ? 'ON' : 'OFF'}
            </button>
        )
    }
}

const container = document.getElementById('app');
const root = createRoot(container!);
root.render(<Toggle />);