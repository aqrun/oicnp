'use client';

import styled from "styled-components";

export const HeaderMenu = styled.div`
  .header-nav-item {
    &.active {
      background-color: var(--color-very-light-purple);
      @apply text-gray-800;
    }
  }
`;