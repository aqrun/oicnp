import 'nprogress/nprogress.css';

import { allRoutes } from '~/routes';
import {
  createBrowserRouter,
  RouterProvider,
} from 'react-router';
import {
  QueryClientProvider,
  QueryClient,
} from '@tanstack/react-query';
import {
  GlobalStyle,
} from './styles/app.styled';

const queryClient = new QueryClient();

export default function App() {
  return (
    <QueryClientProvider
      client={queryClient}
    >
      <GlobalStyle />
      <RouterProvider
        router={createBrowserRouter(allRoutes)}
      />
    </QueryClientProvider>
  )
}

