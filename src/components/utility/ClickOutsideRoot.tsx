import { EventBus } from '../../utils/observable';
import React, { FunctionComponent, useState } from 'react';
import { ClickOutsideContext } from './ClickOutside';

interface Props {
	children: React.ReactNode;
	className?: string;
}

export const ClickOutsideRoot: FunctionComponent<Props> = ({ children, className }) => {
	const [clickOutsideSubject] = useState((): EventBus<React.MouseEvent> => new EventBus());

	const onClick = (e: React.MouseEvent): void => {
		clickOutsideSubject.next(e);
	};

	return (
		<ClickOutsideContext.Provider value={clickOutsideSubject}>
			<div className={className} onClick={onClick}>
				{children}
			</div>
		</ClickOutsideContext.Provider>
	);
};

export default ClickOutsideRoot;
