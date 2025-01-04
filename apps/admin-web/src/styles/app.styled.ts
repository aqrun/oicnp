import { createGlobalStyle } from 'styled-components';

export const GlobalStyle = createGlobalStyle`
  html,body {
    margin: 0;
    padding: 0;
    height: 100%;
  }

  #root {
    height: 100%;
  }

  #nprogress .bar {
    background-color: #00b96b;
  }
`;