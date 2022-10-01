import React, { useCallback, useLayoutEffect } from 'react';
import useResizeObserver from '@react-hook/resize-observer';

export const useSetCSSVarToHeight = (ref: React.RefObject<HTMLElement>, propertyName: string): void => {
    const updateVar = useCallback((height: number): void => {
        document.querySelector('html')!.style.setProperty(propertyName, height + 'px');
    }, [propertyName]);

    useLayoutEffect((): void => {
        updateVar(ref.current!.offsetHeight);
    }, [updateVar, ref]);

    useResizeObserver(ref, (entry): void => updateVar((entry.target as HTMLElement).offsetHeight));
}