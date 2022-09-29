import { createAction } from 'typesafe-actions';

export const loadingStart = createAction('shell/LOADING')<string>();
export const loadingEnd = createAction('shell/LOADING_DONE')<string>();

export const setRouterLocation = createAction('shell/SET_ROUTER_LOCATION', (location?: string) => location)();

export const resetRouterLocation = createAction('shell/RESET_ROUTER_LOCATION')();