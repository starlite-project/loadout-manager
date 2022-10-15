import { Base } from "../base";

export interface RawGeneralUser {
    membershipId: string;
    uniqueName: string;
    normalizedName: string;
    displayName: string;
    profilePicture: number;
    profileTheme: number;
    userTitle: number;
    successMessageFlags: string;
    isDeleted: boolean;
    about: string;
    firstAccess: number | null;
    lastUpdate: number;
    psnDisplayName: string;
    xboxDisplayName: string;
    fbDisplayName: string;
    showActivity: boolean | null;
    locale: string;
}

export class GeneralUser extends Base<RawGeneralUser> {
    public get firstAccess(): Date | null {
        return this._data.firstAccess ? new Date(this._data.firstAccess) : null;
    }

    public get displayName(): string {
        return this._data.displayName;
    }
}