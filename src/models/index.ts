import { invoke } from '@tauri-apps/api/tauri';
import { error } from '../plugins/Log';
import { TokenUtils } from '../utils';

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

export const getActiveToken = async (): Promise<TokenUtils.AuthTokens> => {
    const token = TokenUtils.getToken();

    if (!token) {
        TokenUtils.removeToken();
        throw new FatalTokenError('No auth token, redirect to login');
    }

    const accessTokenIsValid = token && !TokenUtils.hasTokenExpired(token.accessToken);
    console.log(accessTokenIsValid);
    if (accessTokenIsValid) {
        return token;
    }

    const refreshTokenIsValid = token && !TokenUtils.hasTokenExpired(token.refreshToken);
    console.log(refreshTokenIsValid);
    if (!refreshTokenIsValid) {
        TokenUtils.removeToken();
        throw new FatalTokenError('Refresh token invalid, clearing auth tokens and going to login');
    }

    let newToken: TokenUtils.AuthTokens | null = null;
    try {
        newToken = await invoke('refresh_token', { token });
        console.log(newToken);
        TokenUtils.setToken(newToken!);
        return TokenUtils.getToken()!
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