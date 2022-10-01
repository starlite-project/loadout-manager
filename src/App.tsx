import { Navigate, Route, Routes } from 'react-router-dom';
import { PageLoading, Header } from './components';
import { Login, Settings } from './pages';
import { useSelector } from 'react-redux';
import type { RootState } from './store/types';
import { ErrorBoundary, ScrollToTop, ShowPageLoading } from './components/utility';
import React, { Suspense } from 'react';
import { t } from './utils';
import styles from './App.module.scss';

const User = React.lazy(() => import('./components/User'));

export function App(): JSX.Element {
	const needsLogin = useSelector((state: RootState): boolean => state.accounts.needsLogin);

	return (
		<div className={styles.wrapper}>
			<ScrollToTop />
			<PageLoading />
			<Header />
			<ErrorBoundary name="Loadout Manager">
				<Suspense fallback={<ShowPageLoading message={t('Loading.Code')} />}>
					<Routes>
						<Route path="login" element={<Login />} />
						<Route path="user" element={<User />} />
						<Route path="settings" element={<Settings />} />
							{needsLogin ? (
								<Route path="*" element={<Navigate to="/login" />} />
							) : (
								<Route path="*" element={<Navigate to="/user" />} />
							)}
					</Routes>
				</Suspense>
			</ErrorBoundary>
		</div>
	);
}

export default App;
