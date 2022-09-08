import React, { useState } from 'react';
import { Button, Menu, MenuItem } from '@mui/material'

export function DefaultMenu() {
    const [anchor, setAnchor] = useState<null | HTMLElement>(null);
    const open = Boolean(anchor);
    const handleClick = (event: React.MouseEvent<HTMLButtonElement>) => {
        setAnchor(event.currentTarget);
    };

    const handleClose = () => {
        setAnchor(null);
    };

    return (
        <div>
            
        </div>
    )
}

export default DefaultMenu;