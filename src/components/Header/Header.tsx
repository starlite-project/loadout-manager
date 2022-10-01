import React, { type FunctionComponent, useCallback, useState, useRef, useEffect } from 'react';
import { useThunkDispatch } from '../../store/thunk';
import { useLocation } from 'react-router-dom';
import styles from './Header.module.scss';
import { useSetCSSVarToHeight } from '../../utils/hooks';
import clsx from 'clsx';

export const Header: FunctionComponent = () => {
    const dispatch = useThunkDispatch();

    const [dropdownOpen, setDropdownOpen] = useState(false);
    const dropdownToggler = useRef<HTMLAnchorElement>(null);
    const toggleDropdown = useCallback((e: React.MouseEvent | KeyboardEvent) => {
        e.preventDefault();
        setDropdownOpen((state): boolean => !state);
    }, []);

    const hideDropdown = useCallback((): void => {
        setDropdownOpen(false);
    }, []);


    const { pathname } = useLocation();

    useEffect((): void => {
        setDropdownOpen(false);
    }, [dispatch, pathname]);

    const headerRef = useRef<HTMLDivElement>(null);
    useSetCSSVarToHeight(headerRef, '--header-height');

    return (
        <header className={styles.container} ref={headerRef}>
            <div>
                <a
                    ref={dropdownToggler}
                    onClick={toggleDropdown}
                    role="button"
                    aria-haspopup="menu"
                    aria-expanded={dropdownOpen}
                >
                    This is the menu
                </a>
            </div>
        </header>
    )
}

Header.whyDidYouRender = false;

export default Header;