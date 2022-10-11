import type React from "react";

export const scrollToPosition = (options: ScrollToOptions): void => {
    const isSmoothScrollSupported = 'scrollBehavior' in document.documentElement.style;
    isSmoothScrollSupported ? window.scroll(options) : document.scrollingElement!.scrollTop = options.top!;
}

export const scrollToElement = (elem: Element | null): void => {
    if (elem) {
        const headerHeight = parseInt(
            document.querySelector('html')!.style.getPropertyValue('--header-height'),
            10
        );

        const rect = elem.getBoundingClientRect();
        scrollToPosition({
            top: window.scrollY + rect.top - (headerHeight + 6),
            left: 0,
            behavior: 'smooth'
        })
    }
}

export const scrollToHref = (e: React.MouseEvent): void => {
    e.preventDefault();
    const elem = document.getElementById((e.currentTarget as HTMLAnchorElement).hash.slice(1));
    scrollToElement(elem);
}