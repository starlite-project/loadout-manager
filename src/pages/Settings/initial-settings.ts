import { defaultSettings, Settings as BaseSettings } from '@destinyitemmanager/dim-api-types';
import { defaultLanguage } from '../../utils/i18n';


// eslint-disable-next-line @typescript-eslint/no-empty-interface
export interface Settings extends BaseSettings {

}

export const initialSettingsState: Settings = {
    ...defaultSettings,
    language: defaultLanguage()
};