import React, { useState } from 'react';
import { BrowserRouter, Route, Routes } from 'react-router-dom';
import './App.css'
import { getToken } from './token';

export default function App() {
    const [isLoggedIn, setIsLoggedIn] = useState(false);

    return (
        <button onClick={async () => await getToken()}>Login</button>
    );
}