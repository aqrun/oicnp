'use client';

import styled from 'styled-components';

export const SideNavContainer = styled.div`
  width: 200px;
  /* height: calc(100vh - 200px); */
  background-color: white;
  width: 100%;
  position: relative;
  font-size: 16px;
  color: var(--text-color);

  @media (min-width: 64rem) {
    position: sticky;
    width: 200px;
    min-width: 200px;
    top: 80px;
    max-height: 400px;
  }

  .side-nav-item {
    &.active {
      a {
        color: var(--primary);
        background-color: var(--color-very-light-purple);
        font-weight: 500;
      }
    }

    @media (min-width: 64rem) {
      a {
        border-left: 3px solid transparent;
        border-top-left-radius: 0;
        border-bottom-left-radius: 0;
      }
      &.active {
        border-left: 3px solid var(--primary);
      }
    }

    a {
      display: block;
      padding: 10px 17px;
      border-radius: 4px;
      cursor: pointer;

      &:hover {
        color: var(--primary);
        background-color: var(--color-very-light-purple);
        font-weight: 500;
      }
    }
  }
`;