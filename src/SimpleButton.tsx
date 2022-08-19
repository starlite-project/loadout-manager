// import React from 'react';
import { invoke } from '@tauri-apps/api/tauri';

export function Button() {
    return (<button onClick={getApps}>Get Apps</button>)
}

async function getApps(): Promise<void> {
    const p = await invoke('get_bungie_applications');

    console.log(p);
}