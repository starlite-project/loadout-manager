import React, { useEffect, useState } from 'react';
import { BrowserRouter, Route, Routes } from 'react-router-dom';
import './App.css'
import { DefaultMenu, User } from './components';
import { LoginButton, LogoutButton } from './OauthButtons';
import { canRefreshToken, isTokenValid, refreshToken, useToken } from './token';

export const App = () => {
    const [loggedIn, setLoggedIn] = useToken();


    return loggedIn ? (
        // <LogoutButton setLoggedIn={setLoggedIn} />
        <div>
            <LogoutButton setLoggedIn={setLoggedIn} />
            <DefaultMenu />
        </div>
    ) : (
        <LoginButton setLoggedIn={setLoggedIn} />
    )
}

export default App;