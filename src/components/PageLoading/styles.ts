import { css } from '@emotion/css';

export const PageLoading = css`
    position: relative;
    pointer-events: none;
`;

export const PageLoadingEnter = css`
    opacity: 0;
`;

export const PageLoadingEnterActive = css`
    opacity: 1;
    transition: opacity 100ms ease-in 500ms;
`;

export const PageLoadingExit = css`
    > section {
        opacity: 0;
    }
`;