'use client';

import styled from 'styled-components';

export const Container = styled.div`
  a {
    &:hover {
      .lx-img-w {
        &>span {
          transform: scale(1.5);
        }
      }
    }

    .lx-img-w {
      color: var(--color-gray-200);
      width: 70px;
      height: 70px;
      display: flex;

      &>span {
        transition: transform 0.3s;
      }

      svg {
        max-width: 70%;
        max-height: 70%;
      }
    }
  }
`;