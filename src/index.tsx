import React from 'react';
import { createRoot } from 'react-dom/client';
import { attachConsole } from './plugins/Log';
import App from './App';

attachConsole();
const container = document.getElementById('app');
const root = createRoot(container!);
root.render(<App />)