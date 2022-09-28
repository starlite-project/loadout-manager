export interface Token {
	value: string;
	expires: string;
	name: 'access' | 'refresh';
}

export interface AuthTokens {
	accessToken: Token;
	refreshToken?: Token;
	bungieMembershipId: string;
}

const localStorageKey = 'authorization';

export const getToken = (): AuthTokens | null => {
	const tokenString = localStorage.getItem(localStorageKey);
	return tokenString ? JSON.parse(tokenString) : null;
};

export const setToken = (token: AuthTokens): void =>
	localStorage.setItem(localStorageKey, JSON.stringify(token));

export const removeToken = (): void => localStorage.removeItem(localStorageKey);

export const hasValidAuthTokens = (): boolean => {
	const token = getToken();
	if (!token) {
		return false;
	}

	const refreshTokenIsValid = token && !hasTokenExpired(token.refreshToken);
	if (!refreshTokenIsValid) {
		return false;
	}
	return true;
};

export const removeAccessToken = (): void => {
	const token = getToken();
	if (token) {
		token.accessToken.expires = new Date(Date.now()).toISOString();
		setToken(token);
	}
};

export const hasTokenExpired = (token?: Token): boolean => {
	if (!token) {
		return true;
	}

	const expires = getTokenExpiration(token);
	const now = new Date(Date.now());

	return now > expires;
};

const getTokenExpiration = (token?: Token): Date => new Date(token?.expires ?? 0);
