import styled, { createGlobalStyle } from 'styled-components';
import { Space } from 'antd';
import { ANT_PREFIX } from '~/constants';

export const TableActionContainer = styled(Space)`
  column-gap: 0;

  button {
    margin: 0;
    padding: 0;
  }
`;

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

  .${ANT_PREFIX}-btn {
    border-radius: 2px;
  }
  .${ANT_PREFIX}-pagination {
    .${ANT_PREFIX}-pagination-item {
      border-radius: 2px;
    }
  }
  .${ANT_PREFIX}-input-search {
    .${ANT_PREFIX}-input-group {
      .${ANT_PREFIX}-input-affix-wrapper:not(:last-child) {
        border-start-start-radius: 2px;
        border-end-start-radius: 2px;
      }
    }
  }
`;