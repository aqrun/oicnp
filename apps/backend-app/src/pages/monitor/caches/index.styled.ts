'use client';

import styled from 'styled-components';

export const Container = styled.div`
  height: 100%;

  .oic-card-cache-list-w {
    max-width: 25%;
  }
  .oic-card-cache-content-w {
    max-width: 25%;
  }

  @media (min-width: 1600px) {
    .oic-card-cache-list-w {
      max-width: none;
    }
    .oic-card-cache-content-w {
      max-width: none;
    }
  }
`;