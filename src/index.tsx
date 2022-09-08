import React from 'react';
import ReactDOM from 'react-dom/client';
import { attachConsole } from './plugins/Log';
import App from './App';


await attachConsole();

const root = ReactDOM.createRoot(document.getElementById('app')!);

root.render(<App />)