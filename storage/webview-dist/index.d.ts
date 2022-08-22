import { UnlistenFn } from '@tauri-apps/api/event';
export declare class Store {
    private path;
    constructor(path: string);
    set(key: string, value: unknown): Promise<void>;
    get<T>(key: string): Promise<T | null>;
    has(key: string): Promise<boolean>;
    delete(key: string): Promise<boolean>;
    clear(): Promise<void>;
    reset(): Promise<void>;
    keys(): Promise<string[]>;
    values(): Promise<string[]>;
    entries<T>(): Promise<[key: string, value: T][]>;
    length(): Promise<string[]>;
    load(): Promise<void>;
    save(): Promise<void>;
    onKeyChange<T>(key: string, cb: (value: T | null) => void): Promise<UnlistenFn>;
    onChange(cb: (key: string, value: unknown) => void): Promise<UnlistenFn>;
}
