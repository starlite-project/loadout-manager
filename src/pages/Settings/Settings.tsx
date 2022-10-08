import type { FunctionComponent } from 'react';
import { useNavigate } from 'react-router';
import { useThunkDispatch } from '../../store/thunk';
import { removeToken } from '../../utils/token';
import { loggedOut } from '../../store/account/actions';

export const Settings: FunctionComponent = () => {
	const navigate = useNavigate();
	const dispatch = useThunkDispatch();

	const logOut = (): void => {
		removeToken();
		dispatch(loggedOut());
		return navigate('/');
	};

	return (
		<div>
			<>Settings</>
			<button onClick={logOut}>Logout</button>
		</div>
	);
};

export default Settings;
