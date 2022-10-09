import type { FunctionComponent } from 'react';
import { useNavigate } from 'react-router';
import { useThunkDispatch } from '../../store/thunk';
import { removeToken } from '../../utils/token';
import { loggedOut } from '../../store/account/actions';
import './Settings.scss';
import { mapToOptions } from './Select';

const languageOptions = mapToOptions({
	de: 'Deutsch',
	en: 'English',
	es: 'Español (España)',
	'es-mx': 'Español (México)',
	fr: 'Français',
	it: 'Italiano',
	ko: '한국어',
	pl: 'Polski',
	'pt-br': 'Português (Brasil)',
	ru: 'Русский',
	ja: '日本語',
	'zh-cht': '繁體中文', // Chinese (Traditional)
	'zh-chs': '简体中文', // Chinese (Simplified)
});

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
			<button onClick={logOut}>Logout</button>
		</div>
	);
};

export default Settings;
