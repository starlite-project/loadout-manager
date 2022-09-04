import { invoke } from '@tauri-apps/api/tauri';

export class Store {
    public constructor() { }

    public set(key: string, value: unknown): Promise<void> {
        return invoke('plugin:storage|set', { key, value });
    }

    public get<T>(key: string): Promise<T | null> {
        return invoke('plugin:storage|get', { key });
    }

    public has(key: string): Promise<boolean> {
        return invoke('plugin:storage|has', { key });
    }

    public delete(key: string): Promise<boolean> {
        return invoke('plugin:storage|delete', { key });
    }

    public clear(): Promise<void> {
        return invoke('plugin:storage|clear');
    }

    public keys(): Promise<string[]> {
        return invoke('plugin:storage|keys');
    }

    public values(): Promise<unknown[]> {
        return invoke('plugin:storage|values');
    }

    public entries<T>(): Promise<[key: string, value: unknown][]> {
        return invoke('plugin:storage|entries');
    }

    public length(): Promise<number> {
        return invoke('plugin:storage|length');
    }
}