import type { AnyAction } from 'redux';
import type { ThunkAction, ThunkDispatch } from 'redux-thunk';
import type { AccountsState } from './account';
import type { ShellState } from './shell';

export interface RootState {
	readonly accounts: AccountsState;
	readonly shell: ShellState;
}

export type ThunkResult<R = void> = ThunkAction<Promise<R>, RootState, undefined, AnyAction>;
export type LoadoutManagerDispatch = ThunkDispatch<RootState, undefined, AnyAction>;
export type ThunkDispatchProp = {
	dispatch: LoadoutManagerDispatch;
};
