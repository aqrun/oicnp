'use client';

import styled from 'styled-components';

export const SideNavContainer = styled.div`
  background-color: white;
  position: relative;
  width: 100%;
  font-size: 16px;
  color: var(--text-color);

  @media (min-width: 64rem) {
    position: sticky;
    min-width: 200px;
    width: 200px;
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

      &:hover {
        color: var(--primary);
        background-color: var(--color-very-light-purple);
        font-weight: 500;
      }
    }
  }
`;