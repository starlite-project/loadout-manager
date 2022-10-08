import { useEventBusListener } from '../../utils/hooks';
import { EventBus } from '../../utils/observable';
import React, { useCallback, useContext, useEffect, useRef } from 'react';

export const ClickOutsideContext = React.createContext(new EventBus<React.MouseEvent>());

type Props = React.HTMLAttributes<HTMLDivElement> & {
	children: React.ReactNode;
	/** An optional second ref that will be excluded from being considered "outside". This is good for preventing the triggering button from double-counting clicks. */
	extraRef?: React.RefObject<HTMLElement>;
	onClickOutside(event: React.MouseEvent | MouseEvent): void;
};
// eslint-disable-next-line react/prop-types, @typescript-eslint/no-unused-vars
const ClickOutsideInternal = (
	{ onClickOutside, children, extraRef, onClick: _onClick, ...other }: Props,
	ref: React.RefObject<HTMLDivElement> | null
): JSX.Element => {
	const localRef = useRef<HTMLDivElement>(null);
	const wrapperRef = ref || localRef;
	const mouseEvents = useContext(ClickOutsideContext);

	const handleClickOutside = useCallback(
		(event: React.MouseEvent) => {
			const target = event.target as Node;
			if (
				wrapperRef.current &&
				!wrapperRef.current.contains(target) &&
				(!extraRef?.current || !extraRef.current.contains(target))
			) {
				onClickOutside(event);
			}
		},
		[onClickOutside, wrapperRef, extraRef]
	);

	useEventBusListener(mouseEvents, handleClickOutside);

	useEffect(() => {
		const handler = (e: MouseEvent): void => {
			if (e.target === document.body) {
				onClickOutside(e);
			}
		};

		document.addEventListener('click', handler);
		return (): void => document.removeEventListener('click', handler);
	});

	return (
		<div ref={wrapperRef} {...other}>
			{children}
		</div>
	);
};

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
export default React.forwardRef(ClickOutsideInternal);

// export default ClickOutside;
