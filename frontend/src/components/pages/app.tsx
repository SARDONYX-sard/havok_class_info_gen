'use client';

import { Box } from '@mui/material';

import { ExecuteForm } from '@/components/form';
import { Toaster } from '@/components/notifications';
import { useLocale, useToastLimit } from '@/hooks';

export default function App() {
  useLocale();
  useToastLimit(1);

  return (
    <>
      <Box
        component="main"
        sx={{
          display: 'grid',
          minHeight: 'calc(100vh - 56px)',
          placeContent: 'center',
          placeItems: 'center',
          width: '100%',
        }}
      >
        <ExecuteForm />
      </Box>
      <Toaster />
    </>
  );
}
