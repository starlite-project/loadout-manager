import { Navigate, Route, Routes } from 'react-router-dom';
import { Login, PageLoading } from './components';
import { useSelector } from 'react-redux';
import type { RootState } from './store/types';
import { ErrorBoundary, ScrollToTop, ShowPageLoading } from './components/utility';
import React, { Suspense } from 'react';
import { t } from './utils';
import styled from '@emotion/styled';

const User = React.lazy(() => import('./components/User'));

const StyledApp = styled.div`
    padding: 20px;
`;

export function App(): JSX.Element {
    const needsLogin = useSelector((state: RootState): boolean => state.accounts.needsLogin);

    return (
        <StyledApp>
            <ScrollToTop />
            <PageLoading />
            <ErrorBoundary name="Loadout Manager">
                <Suspense fallback={<ShowPageLoading message={t('Loading.Code')} />}>
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
        </StyledApp>
    )
}

export default App;