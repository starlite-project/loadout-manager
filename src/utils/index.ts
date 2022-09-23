export * as TokenUtils from './token';

export const dedupPromise = <T extends unknown[], K>(
    func: (...args: T) => Promise<K>
): (...args: T) => Promise<K> => {
    let promiseCache: Promise<K> | null = null;

    return async (...args: T) => {
        if (promiseCache) {
            return promiseCache;
        }
        promiseCache = func(...args);
        try {
            return await promiseCache;
        } finally {
            promiseCache = null;
        }
    }
}