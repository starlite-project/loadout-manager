import { BrowserRouter } from 'react-router-dom';
import App from './App';
import { Provider } from 'react-redux';
import store from './store';
import { SWRConfig, SWRConfiguration } from 'swr';
import { debug } from './plugins/Log';
import { fetch } from './models';
import React from 'react';

export function Root(): JSX.Element {
	const swrConfig: SWRConfiguration = {
		fetcher: async (key) => {
			await debug(`Making request to route ${key}`);
			return fetch(key);
		},
		revalidateOnMount: false,
		revalidateOnFocus: false,
		revalidateIfStale: true,
		shouldRetryOnError: false,
		suspense: true,
	};

	return (
		<React.StrictMode>
			<SWRConfig value={swrConfig}>
				<BrowserRouter>
					<Provider store={store}>
						<App />
					</Provider>
				</BrowserRouter>
			</SWRConfig>
		</React.StrictMode>
	);
}

export default Root;
