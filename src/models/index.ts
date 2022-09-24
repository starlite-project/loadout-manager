import { invoke } from '@tauri-apps/api/tauri';
import { error } from '../plugins/Log';
import { AuthTokens, getToken, removeToken, hasTokenExpired, setToken } from '../utils/token';

export * from './user';

export interface BungieResponse<T> {
    Response: T;
    ErrorCode: number;
    ThrottleSeconds: number;
    ErrorStatus: string;
    Message: string;
    MessageData: Map<string, string>,
    DetailedErrorTrace: string | null;
}

export const getActiveToken = async (): Promise<AuthTokens> => {
    const token = getToken();

    if (!token) {
        removeToken();
        throw new FatalTokenError('No auth token, redirect to login');
    }

    const accessTokenIsValid = token && !hasTokenExpired(token.accessToken);
    console.log(accessTokenIsValid);
    if (accessTokenIsValid) {
        return token;
    }

    const refreshTokenIsValid = token && !hasTokenExpired(token.refreshToken);
    console.log(refreshTokenIsValid);
    if (!refreshTokenIsValid) {
        removeToken();
        throw new FatalTokenError('Refresh token invalid, clearing auth tokens and going to login');
    }

    let newToken: AuthTokens | null = null;
    try {
        newToken = await invoke('refresh_token', { token });
        console.log(newToken);
        setToken(newToken!);
        return getToken()!
    } catch (e) {
        await error(e as string);
        throw new FatalTokenError('failed to fetch token');
    }
}

export class FatalTokenError extends Error {
    public constructor(message: string) {
        super(message);
        this.name = 'FatalTokenError';
    }
}