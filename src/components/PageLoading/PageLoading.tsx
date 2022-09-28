import { FunctionComponent, ReactElement, useRef } from 'react';
import { useSelector } from 'react-redux';
import type { RootState } from '../../store/types';
import { TransitionGroup, CSSTransition } from 'react-transition-group';
import clsx from 'clsx';
import * as styles from './styles';

const messageSelector = (state: RootState) => {
	const length = state.shell.loadingMessages.length;
	return length ? state.shell.loadingMessages[length - 1] : undefined;
};

const transitionClasses = {
	enter: styles.PageLoading,
	enterActive: styles.PageLoadingEnterActive,
	exit: styles.PageLoadingExit,
} as const;

export const PageLoading: FunctionComponent = (): ReactElement => {
	const message = useSelector(messageSelector);
	const nodeRef = useRef<HTMLDivElement>(null);
	return (
		<TransitionGroup component={null}>
			{message !== null && (
				<CSSTransition
					nodeRef={nodeRef}
					classNames={transitionClasses}
					timeout={{ enter: 600, exit: 300 }}
				>
					<div ref={nodeRef} className={clsx(styles.PageLoading)}>
						<div>{message}</div>
					</div>
				</CSSTransition>
			)}
		</TransitionGroup>
	);
};

export default PageLoading;
