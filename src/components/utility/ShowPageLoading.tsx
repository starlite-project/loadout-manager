import { FunctionComponent, useEffect } from 'react';
import { loadingEnd, loadingStart } from '../../store/shell/actions';
import { useThunkDispatch } from '../../store/thunk';

export const ShowPageLoading: FunctionComponent<{ message: string }> = ({ message }): null => {
	const dispatch = useThunkDispatch();
	useEffect(() => {
		dispatch(loadingStart(message));
		return (): void => {
			dispatch(loadingEnd(message));
		};
	}, [dispatch, message]);
	return null;
}

export default ShowPageLoading;
