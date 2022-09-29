// import styled, { css } from 'styled-components';
import { css } from '@emotion/css';
import styled from '@emotion/styled';

/* eslint-disable-next-line @typescript-eslint/no-namespace */
export const CSSConstants = {
	// Elements
	Arc: '#85c5ec' as const,
	Solar: '#f2721b' as const,
	Void: '#814bcf' as const,

	// Rarieties
	Common: '#dcdcdc' as const,
	Uncommon: '#366e42' as const,
	Rare: '#5076a3' as const,
	Legendary: '#513065' as const,
	Exotic: '#c3a019' as const,

	// Misc
	Xp: '#5ea16a' as const,
	Gold: '#f5dc56' as const,
	get Power(): '#f5dc56' {
		return this.Gold;
	},

	MasterworkBorderColor: '#eade8b' as const,
	DeepsightBorderColor: '#d25336' as const,

	Purple: '#843da4' as const,
	Scarlet: '#da004e' as const,
};

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
