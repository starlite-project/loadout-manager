import React from 'react';
import ReactDOM from 'react-dom/client';
import { attachConsole } from './plugins/Log';
import Root from './Root';


// await attachConsole();

// const root = ReactDOM.createRoot(document.getElementById('app')!);

// root.render(<Root />)

(async () => {
    await attachConsole();

    const root = ReactDOM.createRoot(document.getElementById('app')!);

    root.render(<Root />);
})();