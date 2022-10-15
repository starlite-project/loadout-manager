import type { BungieMembershipType } from "bungie-api-ts/common";
import { Base } from "../base";

export interface RawUserInfoCard {
    supplementalDisplayName: string;
    iconPath: string;
    crossSaveOverride: number;
    applicableMembershipTypes: BungieMembershipType[];
    isPublic: boolean;
    membershipType: BungieMembershipType;
    membershipId: string;
    bungieGlobalDisplayName: string;
    bungieGlobalDisplayNameCode: number | null;
}

export class UserInfoCard extends Base<RawUserInfoCard> {
    public get fullDisplayName(): string {
        return `${this._data.bungieGlobalDisplayName}${this._data.bungieGlobalDisplayNameCode ? `#${this._data.bungieGlobalDisplayNameCode}` : ''}`;
    }
}