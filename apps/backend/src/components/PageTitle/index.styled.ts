'use client';

import styled from 'styled-components';
import { CLASS_PREFIX } from '@/constants';

export const Container = styled.div`
  margin-bottom: 16px;
  display: flex;
  align-items: center;
  font-size: 24px;

  .${CLASS_PREFIX}-icon-back {
    margin-right: 8px;
    cursor: pointer;
  }
  
  h2 {
    margin: 0;
    line-height: 32px;
    color: #111;
    font-size: 24px;
    font-weight: 500;
  }
`;