import i18next, { TOptions } from 'i18next';

export const dedupPromise = <T extends unknown[], K>(
	func: (...args: T) => Promise<K>
): ((...args: T) => Promise<K>) => {
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
	};
};

export const t = (key: string | string[], options?: string | TOptions | undefined): string =>
	i18next.t(key, options);

export function tl<T extends string>(key: T): T {
	return key;
}

export const API_BASE: URL = new URL('https://www.bungie.net/');

export const makeBungieURL = (path: string): URL => new URL(path, API_BASE);