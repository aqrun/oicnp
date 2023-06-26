import '../styles/app.scss';
import '../styles/globals.scss';
import type { AppProps } from 'next/app';
import { useEffect } from 'react';
import { nightMode } from '../utils';
import { FixedButtons } from '../components';
import { RecoilRoot } from 'recoil';
import {
 useCheckIsMobile,
} from '~/hooks';

const AppWidget = ({ Component, ...restProps }: AppProps) => {
  useCheckIsMobile();

  useEffect(() => {
    nightMode();
  }, []);

  return (
    <div className="oic-app">
      <Component {...restProps.pageProps} />
      <FixedButtons />
    </div>
  );
};

function MyApp(appProps: AppProps) {
  
  return (
    <RecoilRoot>
      <AppWidget {...appProps} />
    </RecoilRoot>
  );
}

export default MyApp;
