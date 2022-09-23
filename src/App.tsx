import { Navigate, Route, Routes } from 'react-router-dom';
import './App.module.scss';
import { Login } from './components';
import { useSelector } from 'react-redux';
import type { RootState } from './store/types';
import { ErrorBoundary, ScrollToTop, ShowPageLoading } from './components/utility';
import React, { Suspense } from 'react';

const User = React.lazy(() => import('./components/User'));


export function App(): JSX.Element {
    const needsLogin = useSelector((state: RootState): boolean => state.accounts.needsLogin);

    return (
        <div>
            <ScrollToTop />
            <ErrorBoundary name="Loadout Manager">
                <Suspense fallback={<div>Loading...</div>}>
                    <Routes>
                        <Route path="login" element={<Login />} />
                        <Route path="user" element={<User />} />
                        {needsLogin ? (
                            <Route
                                path="*"
                                element={<Navigate to="/login" />}
                            />
                        ) : (
                            <Route
                                path="*"
                                element={<Navigate to="/user" />}
                            />
                        )
                        }
                    </Routes>
                </Suspense>
            </ErrorBoundary>
        </div>
    )
}

export default App;