// import styled, { css } from 'styled-components';
import { css } from '@emotion/css';
import styled from '@emotion/styled';

/* eslint-disable-next-line @typescript-eslint/no-namespace */
export namespace CSSConstants {
	// Elements
	export const Arc = '#85c5ec' as const;
	export const Solar = '#f2721b' as const;
	export const Void = '#814bcf' as const;

	// Rarieties
	export const Common = '#dcdcdc' as const;
	export const Uncommon = '#366e42' as const;
	export const Rare = '#5076a3' as const;
	export const Legendary = '#513065' as const;
	export const Exotic = '#c3a019' as const;

	// Misc
	export const Xp = '#5ea16a' as const;
	export const Gold = '#f5dc56' as const;
	export const Power = Gold;

	export const MasterworkBorderColor = '#eade8b' as const;
	export const DeepsightBorderColor = '#d25336' as const;

	export const ItemBorderWidth = '1px' as const;

	export const EquippedItemBorder = '1px' as const;
	export const EquipeedItemPadding = '2px' as const;
	export const EquippedItemTotalOutset = `#{2 * (${
		EquippedItemBorder + EquipeedItemPadding
	})}` as const;

	export const BadgeFontSize = '(var(--item-size) / 5)' as const;
	export const BadgeHeight = `(#{${BadgeFontSize}} + 4px)` as const;

	export const Purple = '#843da4' as const;
	export const Scarlet = '#da004e' as const;
}

export const DestinyHeader = css`
	text-transform: uppercase;
	font-weight: 600;
	font-family: Helvetica, Arial, sans-serif;
`;

export const LoadoutManagerButton = styled.a`
	cursor: pointer;
	padding: 4px 10px;
	display: inline-block;
	background-color: rgba(255, 255, 255, 0.2);
	color: white;
	font-size: 12px;
	line-height: calc(16 / 12);
	font-family: 'Open Sans', sans-serif, 'Destiny Symbols';
	text-shadow: 1px 1px 3px rgba(0, 0, 0, 0.25);
	border: 1px solid transparent;
	transition: all 150ms ease-out;
	box-sizing: border-box;
	text-align: center;

	img {
		height: 1.3em;
		width: auto;
		vertical-align: bottom;
		margin: 0 0.3em;
		filter: drop-shadow(0 0 1px black);
		transform: filter 150ms ease-out;
	}

	&:hover,
	&:active,
	&.selected {
		background-color: --purple !important;
		color: black !important;
		img {
			filter: invert(1) drop-shadow(0 0 1px black);
		}
	}

	input {
		display: none;
	}
`;
