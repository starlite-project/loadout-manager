/// <reference types="@welldone-software/why-did-you-render" />
import * as React from 'react';

if (import.meta.env.DEV) {
	const { default: whyDidYouRender } = await import('@welldone-software/why-did-you-render');
	const ReactRedux = await import('react-redux');
	whyDidYouRender(React, {
		include: [/.*/],
		exclude: [/^BrowserRouter/, /^Link/, /^Route/, /^Transition/, /^Insertion/],
		trackHooks: true,
		trackAllPureComponents: true,
		trackExtraHooks: [
			[ReactRedux, 'useSelector'],
			[ReactRedux, 'useDispatch'],
		],
	});
}
