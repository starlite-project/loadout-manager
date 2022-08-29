export interface AuthorizationToken {
    accessToken: string;
    expiresIn: number;
    refreshToken: string;
    refreshExpiresIn: number;
    membershipId: string;
}