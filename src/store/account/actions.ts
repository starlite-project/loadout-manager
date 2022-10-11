import { createAction } from 'typesafe-actions';
import type { DestinyAccount } from './types';

export const accountsLoaded = createAction('accounts/ACCOUNTS_LOADED')<DestinyAccount[]>();

export const setCurrentAccount = createAction('accounts/SET_CURRENT_ACCOUNT')<DestinyAccount | undefined>();

export const loggedOut = createAction('accounts/LOG_OUT')();

export const loggedIn = createAction('accounts/LOG_IN')();
