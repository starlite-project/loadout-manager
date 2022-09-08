use std::fmt::{Display, Formatter, Result as FmtResult};

use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum DestinyComponentType {
	None,
	Profiles = 100,
	VendorReceipts,
	ProfileInventories,
	ProfileCurrencies,
	ProfileProgression,
	PlatformSilver,
	Characters = 200,
	CharacterInventories,
	CharacterProgressions,
	CharacterRenderData,
	CharacterActivities,
	CharacterEquipment,
	ItemInstances = 300,
	ItemObjectives,
	ItemPerks,
	ItemRenderData,
	ItemStats,
	ItemSockets,
	ItemTalentGrids,
	ItemCommonData,
	ItemPlugStates,
	ItemPlugObjectives,
	ItemReusablePlugs,
	Vendors = 400,
	VendorCatagories,
	VendorSales,
	Kiosks = 500,
	CurrencyLookups = 600,
	PresentationNotes = 700,
	Collectables = 800,
	Records = 900,
	Transitory = 1000,
	Metries = 1100,
	StringVariables = 1200,
	Craftables = 1300,
}

impl Display for DestinyComponentType {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		Display::fmt(&(*self as i32), f)
	}
}

#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum BungieMembershipType {
	None = 0,
	TigerXbox,
	TigerPsn,
	TigerSteam,
	TigerBlizzard,
	TigerStadia,
	TigerEgs,
	TigerDemon = 10,
	BungieNext = 254,
	All = -1,
}

impl Display for BungieMembershipType {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		Display::fmt(&(*self as i32), f)
	}
}
