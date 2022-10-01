import type { FunctionComponent } from "react";
import { useNavigate } from "react-router";
import { removeToken } from "../../utils/token";

export const Settings: FunctionComponent = () => {
    const navigate = useNavigate();

    const logOut = (): void => {
        removeToken();
        return navigate('/');
    };

    return (
        <div>
            <>Settings</>
            <button onClick={logOut}>Logout</button>
        </div>
    )
}

export default Settings;