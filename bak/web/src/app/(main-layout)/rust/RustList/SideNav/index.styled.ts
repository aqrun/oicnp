'use client';

import styled from 'styled-components';

export const SideNavContainer = styled.div`
  width: 200px;
  /* height: calc(100vh - 200px); */
  background-color: white;
  min-width: 200px;
  position: sticky;
  top: 80px;
  max-height: 400px;
  font-size: 16px;
  color: var(--text-color);

  .side-nav-item {
    &.active {
      a {
        color: var(--primary);
        background-color: var(--color-very-light-purple);
        border-left: 3px solid var(--primary);
        font-weight: 500;
      }
    }

    a {
      display: block;
      padding: 10px 17px;
      border-top-right-radius: 4px;
      border-bottom-right-radius: 4px;
      border-left: 3px solid transparent;

      &:hover {
        color: var(--primary);
        background-color: var(--color-very-light-purple);
        font-weight: 500;
        border-left: 3px solid var(--primary);
      }
    }
  }
`;