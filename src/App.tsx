import React, { useEffect, useState } from 'react';
import { BrowserRouter, Navigate, Route, Routes } from 'react-router-dom';
import './App.css';
import { DefaultMenu, User, Login } from './components';
import { isLoggedIn } from './utils/token';

export function App(): JSX.Element {
    const needsLoggedIn = !isLoggedIn();

    return (
        <div>
            <Routes>
                <Route path="login" element={<Login />} />
                <Route path="user" element={<User />} />
                {needsLoggedIn ? (
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