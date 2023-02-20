import '../styles/app.scss';
import '../styles/globals.scss';
import type { AppProps } from 'next/app';
import { useEffect } from 'react';
import { nightMode } from '../utils';
import { FixedButtons } from '../components';
import { Provider } from 'react-redux';
import withRedux from 'next-redux-wrapper';
import withReduxSaga from 'next-redux-saga';
import { wrapper } from '../redux/store';

function MyApp({ Component, ...restProps }: AppProps) {
  const { store, props } = wrapper.useWrappedStore(restProps);
  const { pageProps } = props;

  useEffect(() => {
    nightMode();
  }, []);

  return (
    <Provider store={store} >
      <div className="oic-app">
        <Component {...pageProps} />
        <FixedButtons />
      </div>
    </Provider>
  );
}

export default MyApp;
