'use client';

import styled from 'styled-components';
import { CLASS_PREFIX } from '@/constants';

export const Container = styled.div`
  display: flex;
  align-items: center;
  justify-content: space-between;

  .${CLASS_PREFIX}-create-button {
    margin: 0 8px 8px 0;
  }
  .${CLASS_PREFIX}-refresh-button {
    margin: 0 0px 8px 8px;
    padding: 0;
    width: 32px;
  }
  .${CLASS_PREFIX}-filter-search-box {
    margin: 0 8px 8px 0;
    width: 300px;
  }
  .${CLASS_PREFIX}-expand-button {
    margin: 0 8px 8px 0;
  }
`;