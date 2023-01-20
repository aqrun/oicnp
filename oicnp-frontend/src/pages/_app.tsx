import '../styles/app.scss';
import '../styles/globals.css';
import type { AppProps } from 'next/app';
import { useEffect } from 'react';
import { nightMode } from '../utils';
import { FixedButtons } from '../components';

function MyApp({ Component, pageProps }: AppProps) {
  useEffect(() => {
    nightMode();
  }, []);

  return (
    <div className="oic-app">
      <Component {...pageProps} />
      <FixedButtons />
    </div>
  );
}

export default MyApp
