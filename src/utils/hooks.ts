import React, { useCallback, useEffect, useLayoutEffect } from 'react';
import useResizeObserver from '@react-hook/resize-observer';
import type { EventBus, Observable } from './observable';

export const useEventBusListener = <T>(eventBus: EventBus<T> | Observable<T>, subscriberFn: (value: T) => void): void => {
    useEffect(() => eventBus.subscribe(subscriberFn), [eventBus, subscriberFn]);
}

export const useSetCSSVarToHeight = (ref: React.RefObject<HTMLElement>, propertyName: string): void => {
    const updateVar = useCallback((height: number): void => {
        document.querySelector('html')!.style.setProperty(propertyName, height + 'px');
    }, [propertyName]);

    useLayoutEffect((): void => {
        updateVar(ref.current!.offsetHeight);
    }, [updateVar, ref]);

    useResizeObserver(ref, (entry): void => updateVar((entry.target as HTMLElement).offsetHeight));
}