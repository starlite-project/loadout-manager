import React from 'react';
import { createRoot } from 'react-dom/client';
import { SimpleButton } from './SimpleButton';
import { OauthButton } from './OauthButton';

const container = document.getElementById('app');
const alreadyLoggedIn = false;
const root = createRoot(container!);
root.render(<SimpleButton />);
root.render(<OauthButton />)