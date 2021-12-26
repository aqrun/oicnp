import styled, { createGlobalStyle } from 'styled-components';
import { Layout } from 'antd';

export const LayoutMain = styled(Layout)`
  padding: 0 50px;
`

export const Global = createGlobalStyle`
  body {
    > #__next {
      
    }
  }

  img {
    max-width: 100%;
  }
`;
