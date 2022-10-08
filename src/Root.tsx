import { BrowserRouter } from 'react-router-dom';
import App from './App';
import { Provider } from 'react-redux';
import store from './store';
import { SWRConfig, SWRConfiguration } from 'swr';
import { debug } from './plugins/Log';
import { fetch } from './models';
import React from 'react';
import LocationSwitcher from './components/utility/LocationSwitcher';
import { DndProvider, MouseTransition, MultiBackendOptions, TouchTransition } from 'react-dnd-multi-backend';
import { TouchBackend } from 'react-dnd-touch-backend';
import { HTML5Backend } from 'react-dnd-html5-backend';

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

	const dndOptions: MultiBackendOptions = {
		backends: [
			{ id: 'html5', backend: HTML5Backend, transition: MouseTransition },
			{
				id: 'touch',
				backend: TouchBackend,
				transition: TouchTransition,
				options: { delayTouchStart: 150 }
			}
		]
	}

	return (
		<React.StrictMode>
			<SWRConfig value={swrConfig}>
				<BrowserRouter>
					<Provider store={store}>
						<LocationSwitcher />
						<DndProvider options={dndOptions}>
							<App />
						</DndProvider>
					</Provider>
				</BrowserRouter>
			</SWRConfig>
		</React.StrictMode>
	);
}

export default Root;
