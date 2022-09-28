import { combineReducers, Reducer } from 'redux';
import { accounts } from './account';
import { shell } from './shell';
import type { RootState } from './types';

const reducer: Reducer<RootState> = (state, action) => {
	return combineReducers({
		accounts,
		shell,
	})(state, action);
};

export default reducer;
