import styled from 'styled-components';
import { CSSConstants, DestinyHeader, LoadoutManagerButton } from '../../main-styles';

export const Billboard = styled.div`
    display: flex;
    flex-direction: column;
    align-items: stretch;
    background-color: rgba(0, 0, 0, 0.6);
    color: white;
    max-width: 800px;
    border-top: 5px solid #888;
    box-shadow: 0 0 2px rgba(0, 0, 0, 0.5);
    padding: 1rem 3rem;
    text-align: center;
    z-index: 99999;
    white-space: pre-wrap;
    box-sizing: border-box;
    margin: auto;

    h1 {
        ${DestinyHeader}
        margin: 0;
        text-align: center;
    }
`;

export const AuthButton = styled(LoadoutManagerButton)`
    font-size: 1rem;
    font-weight: bold;
    text-align: center;
    padding: 1em 3em;
    background-color: ${CSSConstants.Purple};
    color: black;
    text-shadow: none;
    &:hover {
        transform: scale(1.05)
    }
`;