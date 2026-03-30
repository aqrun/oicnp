'use client';

import styled from 'styled-components';

export const Container = styled.div`
  @media (min-width: 64rem) {
    &:nth-child(even) {
      .poem-list-item {
        margin-left: 8px;
      }
    }
    &:nth-child(odd) {
      .poem-list-item {
        margin-right: 8px;
      }
    }
  }
`;