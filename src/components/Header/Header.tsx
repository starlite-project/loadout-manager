import React, { type FunctionComponent, useCallback, useState, useRef } from 'react';
import { useThunkDispatch } from '../../store/thunk';

export const Header: FunctionComponent = () => {
    const dispatch = useThunkDispatch();

    const [dropdownOpen, setDropdownOpen] = useState(false);

    const toggleDropdown = useCallback((e: React.MouseEvent | KeyboardEvent) => {
        e.preventDefault();
        setDropdownOpen((state): boolean => !state);
    }, []);

    const hideDropdown = useCallback((): void => {
        setDropdownOpen(false);
    }, []);

    const headerRef = useRef<HTMLDivElement>(null);

    return (
        <header>
            <div>
                <a>
                    
                </a>
            </div>
        </header>
    )
}

export default Header;