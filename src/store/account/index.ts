import { ActionType, getType } from 'typesafe-actions';
import { hasValidAuthTokens } from '../../utils/token';
import * as actions from './actions';
import type { Reducer } from 'redux';

export interface AccountsState {
    readonly needsLogin: boolean;
}

export type AccountsAction = ActionType<typeof actions>;

const initialState: AccountsState = {
    needsLogin: !hasValidAuthTokens()
};

export const accounts: Reducer<AccountsState, AccountsAction> = (
    state: AccountsState = initialState,
    action: AccountsAction
) => {
    switch (action.type) {
        case getType(actions.loggedOut):
            return {
                ...state,
                needsLogin: true,
            };
        case getType(actions.loggedIn):
            return {
                ...state,
                needsLogin: false,
            }
        default:
            return state
    }
}