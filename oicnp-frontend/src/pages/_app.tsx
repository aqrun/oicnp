import '../styles/app.scss';
import '../styles/globals.scss';
import type { AppProps } from 'next/app';
import { useEffect } from 'react';
import { nightMode } from '../utils';
import { FixedButtons } from '../components';
import { Provider } from 'react-redux';
import { wrapper } from '../redux/store';
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
  const { store, props } = wrapper.useWrappedStore(appProps);

  return (
    <RecoilRoot>
      <Provider store={store} >
        <AppWidget {...appProps} />
      </Provider>
    </RecoilRoot>
  );
}

export default MyApp;
