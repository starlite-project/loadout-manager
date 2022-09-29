import './wdyr';
import React from 'react';
import ReactDOM from 'react-dom/client';
import { attachConsole } from './plugins/Log';
import Root from './Root';
import { initi18n } from './utils/i18n';

const consolePromise = attachConsole();
const i18nPromise = initi18n();

(async (): Promise<void> => {
	const root = ReactDOM.createRoot(document.getElementById('app')!);

	await Promise.all([consolePromise, i18nPromise]);

	root.render(<Root />);
})();
