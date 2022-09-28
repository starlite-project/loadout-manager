import { createAction } from 'typesafe-actions';

export const loadingStart = createAction('shell/LOADING')<string>();
export const loadingEnd = createAction('shell/LOADING_DONE')<string>();
