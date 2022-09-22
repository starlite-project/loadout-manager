import { Navigate, Route, Routes } from 'react-router-dom';
import './App.module.scss';
import { User, Login } from './components';
import { useSelector } from 'react-redux';
import type { RootState } from './store/types';

export function App(): JSX.Element {
    const needsLogin = useSelector((state: RootState): boolean => state.accounts.needsLogin);

    return (
        <div>
            <Routes>
                <Route path="login" element={<Login />} />
                <Route path="user" element={<User />} />
                {needsLogin ? (
                    <Route
                        path="*"
                        element={<Navigate to="/login" />}
                    />
                ) : (
                    <Route
                        path="*"
                        element={<Navigate to="/user" />}
                    />
                )
                }
            </Routes>
        </div>
    )
}

export default App;