'use client';

import { Box } from '@mui/material';

import { TranslationList } from '@/components/lists';
import { Toaster } from '@/components/notifications';
import { useLocale } from '@/hooks';

import packageJson from '@/../../package.json';

export default function Settings() {
  useLocale();

  return (
    <Box
      component="main"
      sx={{
        alignItems: 'center',
        display: 'flex',
        flexDirection: 'column',
        justifyContent: 'center',
        minHeight: 'calc(100vh - 56px)',
        width: '100%',
      }}
    >
      <Toaster position="bottom-right" reverseOrder={false} />

      <div
        style={{
          display: 'flex',
          justifyContent: 'space-between',
          marginTop: '10px',
          overflowX: 'auto',
          width: '95%',
        }}
      >
        <TranslationList />
      </div>
      <Help />
    </Box>
  );
}

const Help = () => {
  const handleClick = () => open(packageJson.homepage);
  return (
    <div
      style={{
        display: 'flex',
        justifyContent: 'space-around',
        marginTop: '10px',
        width: '55%',
      }}
    >
      <div>Version: {packageJson.version}</div>
      <div>
        Source:{' '}
        <a style={{ cursor: 'pointer', color: '#00c2ff' }} onClick={handleClick} onKeyDown={handleClick}>
          GitHub
        </a>
      </div>
    </div>
  );
};
