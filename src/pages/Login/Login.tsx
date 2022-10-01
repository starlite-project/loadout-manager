import { invoke } from '@tauri-apps/api/tauri';
import { error } from '../../plugins/Log';
import { useNavigate } from 'react-router-dom';
import { type AuthTokens, setToken } from '../../utils/token';
import { useThunkDispatch } from '../../store/thunk';
import { loggedIn } from '../../store/account/actions';
import type { FunctionComponent } from 'react';
import { t } from '../../utils';
import React from 'react';
import styles from './Login.module.scss';

const InternalLogin: FunctionComponent = () => {
	const navigate = useNavigate();
	const dispatch = useThunkDispatch();

	const onLoginClick = async (e: React.MouseEvent): Promise<void> => {
		e.preventDefault();
		try {
			const authToken = (await invoke('get_authorization_code')) as AuthTokens;
			setToken(authToken);
			dispatch(loggedIn());
		} catch (e) {
			await error(e as string);
		} finally {
			navigate('/');
		}
	};

	return (
		<div className={styles.billboard}>
			<a className={styles.auth} rel="noopener noreferrer" onClick={onLoginClick}>
				{t('Views.Login.Auth')}
			</a>
		</div>
	);
};

export const Login = React.memo(InternalLogin);

export default Login;
