import { removeToken } from "../../utils/token";
import type { ThunkResult } from "../types";
import * as actions from './actions';

export const logOut = (): ThunkResult => {
    return async (dispatch) => {
        removeToken();
        dispatch(actions.loggedOut());
    }
}