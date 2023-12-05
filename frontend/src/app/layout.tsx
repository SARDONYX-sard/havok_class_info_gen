import { type Metadata } from 'next';
import dynamic from 'next/dynamic';
import { Inter } from 'next/font/google';

import Loading from '@/components/pages/loading';
import '@/utils/translation';

import '@/app/globals.css';
import tauriConfig from '@/../../src-tauri/tauri.conf.json';

const inter = Inter({ subsets: ['latin'] });

const Menu = dynamic(() => import('@/components/navigation'), {
  loading: () => <Loading />,
  ssr: false,
});
const ThemeProvider = dynamic(() => import('@/components/providers/theme'), {
  loading: () => <Loading />,
  ssr: false,
});

export const metadata: Metadata = {
  title: tauriConfig.package.productName,
  description: '',
};

type Props = Readonly<{
  children: React.ReactNode;
}>;
export default function RootLayout({ children }: Props) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <ThemeProvider>
          {children}
          {/* To prevents the conversion button from being hidden because the menu is fixed. */}
          <div style={{ height: '56px' }}></div>
          <Menu />
        </ThemeProvider>
      </body>
    </html>
  );
}
