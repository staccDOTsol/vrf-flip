import React from 'react';
import { Provider, useDispatch } from 'react-redux';
import { ThunkDispatch } from '../types';
import useApi, { ApiProvider } from './providers/ApiProvider';
import store from './store';
import { setUserBalance } from './store/gameStateReducer';
import { log } from './store/hudLoggerReducer';
import { StrataProviders } from 'strata-foundation-react-xnft';
const DataLayer: React.FC<React.PropsWithChildren> = (props) => (
  // Store must be provided before ApiProvider so that ApiProvider can dispatch results.
  <Provider store={store}>
    <StrataProviders>
    <ApiProvider>{props.children} </ApiProvider></StrataProviders>
  </Provider>
);

const useThunkDispatch = (): ThunkDispatch => useDispatch();

export const hooks = {
  useApi,
  useThunkDispatch,
};

export const thunks = {
  log,
  setUserBalance,
};

export type Store = ReturnType<typeof store.getState>;

export default DataLayer;
