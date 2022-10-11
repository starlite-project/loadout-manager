import { ActionType, getType } from 'typesafe-actions';
import { hasValidAuthTokens } from '../../utils/token';
import * as actions from './actions';
import type { Reducer } from 'redux';
import type { DestinyAccount } from './types';
import { deepEqual } from 'fast-equals';

export interface AccountsState {
	readonly accounts: DestinyAccount[];
	readonly currentAccount: number;
	readonly loaded: boolean;
	readonly needsLogin: boolean;
}

export type AccountsAction = ActionType<typeof actions>;

const initialState: AccountsState = {
	accounts: [],
	currentAccount: -1,
	loaded: false,
	needsLogin: !hasValidAuthTokens(),
};

export const accounts: Reducer<AccountsState, AccountsAction> = (
	state: AccountsState = initialState,
	action: AccountsAction
) => {
	switch (action.type) {
		case getType(actions.accountsLoaded):
			return {
				...state,
				accounts: deepEqual(action.payload, state.accounts) ? state.accounts : action.payload || [],
				loaded: true,
			}
		case getType(actions.setCurrentAccount):
			// eslint-disable-next-line no-case-declarations
			const newCurrentAccount = action.payload ? state.accounts.indexOf(action.payload) : -1;
			return newCurrentAccount !== state.currentAccount
				? {
					...state,
					currentAccount: newCurrentAccount,
				} : state;
		case getType(actions.loggedOut):
			return {
				...initialState,
				needsLogin: true,
			};
		case getType(actions.loggedIn):
			return {
				...state,
				needsLogin: false,
			};
		default:
			return state;
	}
};
