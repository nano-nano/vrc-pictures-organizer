import React from 'react';
import ReactDOM from 'react-dom/client';
import AppSettingsPage from './features/app-settings/components/AppSettingsPage';

import { MantineProvider } from '@mantine/core';
import '@mantine/core/styles.css';
import '@mantine/dates/styles.css';
import { Notifications } from '@mantine/notifications';
import '@mantine/notifications/styles.css';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <MantineProvider>
      <Notifications containerWidth="250px" />
      <AppSettingsPage />
    </MantineProvider>
  </React.StrictMode>
);
