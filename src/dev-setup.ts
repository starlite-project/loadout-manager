/// <reference types="@welldone-software/why-did-you-render" />
if (import.meta.env.DEV) {
	const { default: whyDidYouRender } = await import('@welldone-software/why-did-you-render');
	const [React, ReactRedux] = await Promise.all([
		import('react').then(({ default: React }) => React),
		import('react-redux'),
	]);
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
