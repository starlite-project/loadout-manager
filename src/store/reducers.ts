import { combineReducers, Reducer } from 'redux';
import { accounts } from './account';
import type { RootState } from './types';

const reducer: Reducer<RootState> = (state, action) => {
    return combineReducers({
        accounts
    })(state, action)
}

export default reducer;