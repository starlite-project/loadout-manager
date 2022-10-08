import { BungieMembershipType } from 'bungie-api-ts/destiny2';
import { faXbox, faPlayStation, faSteam, battleNetIcon, stadiaIcon, epicIcon } from '../../assets/icons';

const PLATFORM_LABELS = {
    [BungieMembershipType.TigerXbox]: 'Xbox',
    [BungieMembershipType.TigerPsn]: 'PlayStation',
    [BungieMembershipType.TigerBlizzard]: 'Blizzard',
    [BungieMembershipType.TigerDemon]: 'Demon',
    [BungieMembershipType.TigerSteam]: 'Steam',
    [BungieMembershipType.TigerStadia]: 'Stadia',
    [BungieMembershipType.TigerEgs]: 'Epic',
    [BungieMembershipType.BungieNext]: 'Bungie.net',
};

export const PLATFORM_ICONS = {
    [BungieMembershipType.TigerXbox]: faXbox,
    [BungieMembershipType.TigerPsn]: faPlayStation,
    [BungieMembershipType.TigerBlizzard]: battleNetIcon,
    [BungieMembershipType.TigerDemon]: 'Demon',
    [BungieMembershipType.TigerSteam]: faSteam,
    [BungieMembershipType.TigerStadia]: stadiaIcon,
    [BungieMembershipType.TigerEgs]: epicIcon,
    [BungieMembershipType.BungieNext]: 'Bungie.net',
  };

export interface DestinyAccount {
    readonly displayName: string;
    readonly originalPlatformType: BungieMembershipType;
    readonly platformLabel: string;
    readonly membershipId: string;
    readonly platforms: BungieMembershipType[];
    readonly lastPlayed?: Date;
}