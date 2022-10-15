import type { BungieMembershipType } from "bungie-api-ts/common";
import { makeBungieURL } from "../../utils";
import { Base, IconURL } from "../base";
import { RawUserInfoCard, UserInfoCard } from "../user/info";

export interface RawDestinyProfileUserInfoCard {
    isOverridden: boolean;
    isCrossSavePrimary: boolean;
    supplementalDisplayName: string;
    iconPath: string;
    crossSaveOverride: number;
    applicableMembershipTypes: BungieMembershipType[];
    isPublic: boolean;
    membershipType: BungieMembershipType;
    membershipId: string;
    displayName: string;
    bungieGlobalDisplayName: string;
}

export class DestinyProfileUserInfoCard extends Base<RawDestinyProfileUserInfoCard> implements IconURL {
    public iconURL(): URL {
        return makeBungieURL(this._data.iconPath);
    }
}

export interface RawDestinyLinkedProfilesResponse {
    profiles: RawDestinyProfileUserInfoCard[];
    bnetMembership: RawUserInfoCard;
}

export class DestinyLinkedProfilesResponse extends Base<RawDestinyLinkedProfilesResponse> {
    public get profiles(): DestinyProfileUserInfoCard[] {
        return this._data.profiles.map((profile): DestinyProfileUserInfoCard => new DestinyProfileUserInfoCard(profile));
    }

    public get bnetMembership(): UserInfoCard {
        return new UserInfoCard(this._data.bnetMembership);
    }
}