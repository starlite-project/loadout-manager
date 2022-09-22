import { useEffect, useState } from 'react';
import { Navigate, Route, Routes } from 'react-router-dom';
import './App.module.scss';
import { User, Login } from './components';

export function App(): JSX.Element {
    return (
        <div>
            <Routes>
                <Route path="login" element={<Login />} />
                <Route path="user" element={<User />} />
                {!false ? (
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