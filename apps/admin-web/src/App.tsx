import { BrowserRouter as Router } from 'react-router';
import AppRoutes from '~/routes';
import {
  GlobalStyle,
} from './styles/app.styled';

export default function App() {
  return (
    <>
      <GlobalStyle />
      <Router>
        <AppRoutes />
      </Router>
    </>
  )
}

