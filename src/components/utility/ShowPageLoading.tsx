import { useEffect } from 'react';
import { loadingEnd, loadingStart } from '../../store/shell/actions';
import { useThunkDispatch } from '../../store/thunk';

export function ShowPageLoading({ message }: { message: string }): null {
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
