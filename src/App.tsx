import React, { useEffect, useState } from 'react';
import { BrowserRouter, Route, Routes } from 'react-router-dom';
import './App.css';
import { DefaultMenu, User } from './components';
import { OAuthButton } from './components/OAuthButton';

export function App(): JSX.Element {
    return (
        <div>
            <Routes>
            </Routes>
        </div>
    )
}

export default App;