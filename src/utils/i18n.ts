import i18next from 'i18next';
import en from '../locale/en.json';
import resourcesToBackend from 'i18next-resources-to-backend';

interface LangInfo {
    pluralOverride: boolean;
    latinBased: boolean;
}

export const LOADOUT_MANAGER_LANG_INFOS: Record<string, LangInfo> = {
    en: { pluralOverride: false, latinBased: true },
};

const LOADOUT_MANAGER_LANGS = Object.keys(LOADOUT_MANAGER_LANG_INFOS);

export function defaultLanguage(): string {
    const storedLanguage = localStorage.getItem('loadoutManagerLanguage');
    if (storedLanguage && LOADOUT_MANAGER_LANGS.includes(storedLanguage)) {
        return storedLanguage;
    }

    const browserLang = (window.navigator.language || 'en').toLowerCase();
    return LOADOUT_MANAGER_LANGS.find((lang) => browserLang.startsWith(lang)) || 'en';
}

export function initi18n(): Promise<unknown> {
    const lang = defaultLanguage();
    return new Promise((resolve, reject) => {
        i18next.use(resourcesToBackend((language, _namespace, callback) => {
            import(`../locale/${language}.json`)
                .then((resources) => {
                    callback(null, resources);
                }).catch((error) => {
                    callback(error, null);
                });
        })).init({
            initImmediate: true,
            compatibilityJSON: 'v3',
            debug: __LOADOUT_MANAGER_FLAVOR__ == 'dev',
            lng: lang,
            fallbackLng: 'en',
            supportedLngs: LOADOUT_MANAGER_LANGS,
            load: 'currentOnly',
            interpolation: {
                escapeValue: false,
                format(val: string, format) {
                    switch (format) {
                        case 'pct':
                            return `${Math.min(100, Math.floor(100 * parseFloat(val)))}%`;
                        case 'humanBytes':
                            const size = parseInt(val, 10);
                            if (size <= 0) {
                                return '0B';
                            }
                            const i = Math.floor(Math.log(size) / Math.log(1024));
                            return `${(size / Math.pow(1024, i)).toFixed(2)} ${['B', 'KB', 'MB', 'GB', 'TB'][i]}`;
                        case 'number':
                            return parseInt(val, 10).toLocaleString();
                        default:
                            return val;
                    }
                }
            },
            backend: {
                loadPath([lng]: string[]) {
                    const path = {
                        en
                    }[lng] as unknown as string;
                    if (!path) {
                        throw new Error(`unsupported language ${lng}`);
                    }
                    return path;
                }
            },
            returnObjects: true,
        }, (error) => {
            if (error) {
                console.log(error);
                reject(error)
            } else {
                resolve(undefined)
            }
        })
    })
}