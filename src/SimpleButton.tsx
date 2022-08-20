import React from 'react';
import { invoke } from '@tauri-apps/api/tauri';

export class SimpleButton extends React.Component<{}, { data: string, fetched: boolean }> {
    public state = { data: "", fetched: false };

    public render(): React.ReactNode {
        return (
            <div>
                <button onClick={async () => await this.fetch()}>Click me!</button>

                {this.state.fetched && (
                    <div>{this.state.data}</div>
                )}
            </div>
        )
    }

    private async fetch() {
        if (this.state.fetched) {
            return;
        }
        const data = await invoke('get_bungie_applications')

        this.setState((state) => ({
            data: JSON.stringify(data as any, null, 4),
            fetched: true,
        }));
    }
}