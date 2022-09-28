import { applyMiddleware, compose } from 'redux';
import { createStore } from '@reduxjs/toolkit';
import type { RootState } from './types';
import allReducers from './reducers';
import thunk from 'redux-thunk';

// const store = createStore<RootState, any, {}, {}>(allReducers, compose(applyMiddleware(thunk)));

// eslint-disable-next-line @typescript-eslint/no-explicit-any, @typescript-eslint/ban-types
const store = createStore<RootState, any, {}, {}>(allReducers, compose(applyMiddleware(thunk)));

export default store;
