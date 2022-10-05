import React, { type FunctionComponent, useCallback, useState, useRef, useEffect } from 'react';
import { useThunkDispatch } from '../../store/thunk';
import { NavLink, useLocation } from 'react-router-dom';
import styles from './Header.module.scss';
import { useSetCSSVarToHeight } from '../../utils/hooks';
import { ClickOutside } from '../utility';
import clsx from 'clsx';
import { t } from '../../utils';
import { CSSTransition, TransitionGroup } from 'react-transition-group';

const transitionClasses = {
    enter: styles.dropdownEnter,
    enterActive: styles.dropdownEnterActive,
    exit: styles.dropdownExit,
    exitActive: styles.dropdownExitActive
} as const;

export const Header: FunctionComponent = () => {
    const dispatch = useThunkDispatch();

    const [dropdownOpen, setDropdownOpen] = useState(false);
    const dropdownToggler = useRef<HTMLAnchorElement>(null);
    const toggleDropdown = useCallback((e: React.MouseEvent | KeyboardEvent) => {
        e.preventDefault();
        setDropdownOpen((state): boolean => !state);
    }, []);

    const hideDropdown = useCallback((): void => {
        console.log("hiding dropdown");
        setDropdownOpen(false);
    }, []);


    const { pathname } = useLocation();

    useEffect((): void => {
        setDropdownOpen(false);
    }, [dispatch, pathname]);

    const headerRef = useRef<HTMLDivElement>(null);
    useSetCSSVarToHeight(headerRef, '--header-height');

    const dropdownRef = useRef<HTMLDivElement>(null);

    const navLinkClassName = ({ isActive }: { isActive: boolean }): string =>
        clsx(styles.menuItem, { [styles.active]: isActive });

    const lmLinks = (
        <>
            <NavLink to="/about" className={navLinkClassName}>
                {t('Header.About')}
            </NavLink>
        </>
    )

    return (
        <header className={styles.container} ref={headerRef}>
            <div className={styles.header}>
                <a
                    className={clsx(styles.menuItem)}
                    ref={dropdownToggler}
                    onClick={toggleDropdown}
                    role="button"
                    aria-haspopup="menu"
                    aria-label={t('Header.Menu')}
                    aria-expanded={dropdownOpen}
                >
                    This is the menu
                </a>
                <TransitionGroup component={null}>
                    {dropdownOpen && (
                        <CSSTransition
                            nodeRef={dropdownRef}
                            classNames={transitionClasses}
                            timeout={{ enter: 500, exit: 500 }}
                        >
                            <ClickOutside
                                ref={dropdownRef}
                                extraRef={dropdownToggler}
                                key="dropdown"
                                className={styles.dropdown}
                                onClickOutside={hideDropdown}
                                role="menu">
                                Dropped down
                            </ClickOutside>
                        </CSSTransition>
                    )}
                </TransitionGroup>
            </div>
        </header>
    )
}

Header.whyDidYouRender = false;

export default Header;