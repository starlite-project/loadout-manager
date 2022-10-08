import type { Reducer } from 'redux';
import { ActionType, getType } from 'typesafe-actions';
import * as actions from './actions';

export interface ShellState {
	readonly loadingMessages: string[];
	readonly routerLocation?: string;
}

export type ShellAction = ActionType<typeof actions>;

const initialState: ShellState = {
	loadingMessages: [],
	routerLocation: '',
};

export const shell: Reducer<ShellState, ShellAction> = (
	state: ShellState = initialState,
	action: ShellAction
): ShellState => {
	switch (action.type) {
		case getType(actions.loadingStart):
			return {
				...state,
				loadingMessages: [...new Set([...state.loadingMessages, action.payload])],
			};
		case getType(actions.loadingEnd):
			return {
				...state,
				loadingMessages: state.loadingMessages.filter((m): boolean => m !== action.payload),
			};
		case getType(actions.setRouterLocation):
			return {
				...state,
				routerLocation: action.payload,
			};
		case getType(actions.resetRouterLocation):
			return {
				...state,
				routerLocation: undefined,
			};
		default:
			return state;
	}
};
