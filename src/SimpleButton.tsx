import React from 'react';
import { invoke } from '@tauri-apps/api/tauri';

// export function Button() {
//     return (<button onClick={getApps}>Get Apps</button>)
// }

// async function getApps(): Promise<string> {
//     const p = await invoke('get_bungie_applications') as object;

//     return JSON.stringify(p);
// }

export class SimpleButton extends React.Component<{}, { data: string, fetched: boolean }> {
    public state = { data: "", fetched: false };

    public render(): React.ReactNode {
        // return (
        //     <button onClick={async () => await this.fetch()}>
        //         {this.state.data}
        //     </button>
        // )
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
            data: JSON.stringify(data as any),
            fetched: true,
        }));
    }
}