import { createAction } from 'typesafe-actions';

export const loggedOut = createAction('accounts/LOG_OUT')();

export const loggedIn = createAction('accounts/LOG_IN')();