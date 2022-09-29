import { FunctionComponent, useEffect } from 'react';
import { useDispatch, useSelector } from 'react-redux';
import { useNavigate } from 'react-router';
import { routerLocationSelector } from '../../store/shell/selectors';
import { resetRouterLocation } from '../../store/shell/actions';

export const LocationSwitcher: FunctionComponent = (): null => {
    const location = useSelector(routerLocationSelector);
    const navigate = useNavigate();
    const dispatch = useDispatch();

    useEffect((): void => {
        if (location) {
            navigate(location);
            dispatch(resetRouterLocation());
        }
    }, [dispatch, location, navigate]);

    return null;
}

export default LocationSwitcher;