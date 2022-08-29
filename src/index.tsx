import React from 'react';
import { createRoot } from 'react-dom/client';
// import { OauthButton } from './OauthButton';
import App from './App';

const container = document.getElementById('app');
const alreadyLoggedIn = false;
const root = createRoot(container!);
root.render(<App />)