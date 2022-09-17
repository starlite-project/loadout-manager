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