import { invoke } from '@tauri-apps/api/tauri';
import { UnlistenFn } from '@tauri-apps/api/event';
import { appWindow } from '@tauri-apps/api/window';

interface ChangePayload<T> {
    path: string;
    key: string;
    value: T | null;
}

export class Store {
    public constructor(private path: string) { }

    public set(key: string, value: unknown): Promise<void> {
        return invoke<void>('plugin:store|get', {
            path: this.path,
            key,
            value
        })
    }

    public get<T>(key: string): Promise<T | null> {
        return invoke('plugin:store|get', {
            path: this.path,
            key
        })
    }

    public has(key: string): Promise<boolean> {
        return invoke('plugin:store|has', {
            path: this.path,
            key
        })
    }

    public delete(key: string): Promise<boolean> {
        return invoke('plugin:store|delete', {
            path: this.path,
            key
        })
    }

    public clear(): Promise<void> {
        return invoke('plugin:store|clear', {
            path: this.path
        })
    }

    public reset(): Promise<void> {
        return invoke('plugin:store|reset', { path: this.path })
    }

    public keys(): Promise<string[]> {
        return invoke('plugin:store|keys', {
            path: this.path
        })
    }

    public values(): Promise<string[]> {
        return invoke('plugin:store|values', { path: this.path })
    }

    public entries<T>(): Promise<[key: string, value: T][]> {
        return invoke('plugin:store|entries', {
            path: this.path
        })
    }

    public length(): Promise<string[]> {
        return invoke('plugin:store|length', {
            path: this.path
        })
    }

    public load(): Promise<void> {
        return invoke('plugin:store|load', {
            path: this.path
        })
    }

    public save(): Promise<void> {
        return invoke('plugin:store|save', {
            path: this.path
        })
    }

    public onKeyChange<T>(key: string, cb: (value: T | null) => void): Promise<UnlistenFn> {
        return appWindow.listen<ChangePayload<T>>('store://change', event => {
            if (event.payload.path === this.path && event.payload.key === key) {
                cb(event.payload.value)
            }
        })
    }

    public onChange(cb: (key: string, value: unknown) => void): Promise<UnlistenFn> {
        return appWindow.listen<ChangePayload<unknown>>('store://change', event => {
            if (event.payload.path === this.path) {
                cb(event.payload.key, event.payload.value);
            }
        });
    }
}