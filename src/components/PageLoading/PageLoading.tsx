import { FunctionComponent, ReactElement, useRef } from 'react';
import { useSelector } from 'react-redux';
import type { RootState } from '../../store/types';
import { TransitionGroup, CSSTransition } from 'react-transition-group';
import clsx from 'clsx';
import styles from './PageLoading.module.scss';

const messageSelector = (state: RootState): string | undefined => {
	const length = state.shell.loadingMessages.length;
	return length ? state.shell.loadingMessages[length - 1] : undefined;
};

const transitionClasses = {
	enter: styles.pageLoadingEnter,
	enterActive: styles.pageLoadingEnterActive,
	exit: styles.pageLoadingExit,
	exitActive: styles.pageLoadingExitActive,
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
					<div ref={nodeRef} className={clsx(styles.pageLoading)}>
						<div>{message}</div>
					</div>
				</CSSTransition>
			)}
		</TransitionGroup>
	);
};

export default PageLoading;
