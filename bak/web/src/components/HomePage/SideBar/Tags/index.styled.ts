'use client';

import styled from 'styled-components';

export const TagsContainer = styled.div`
  a {
    width: 33.33%;
    color: #333;
    display: block;
    margin-bottom: 8px;
    word-break: break-all;

    &:hover {
      .lx-tags-item {
        border-color: var(--primary);
        background-color: var(--color-very-light-purple);
        color: var(--primary);
      }
    }
    &:nth-child(3n+3) {
      .lx-tags-item {
        margin-right: 0;
      }
    }
    &:nth-child(3n+1) {
      .lx-tags-item {
        margin-left: 0;
      }
    }
  }
  .lx-tags-item {
    margin: 0 4px;
    display: block;
    border-radius: 4px;
    text-align: center;
    border: 1px solid #e2e2e2;
    padding: 0px 8px;
    height: 34px;
    line-height: 34px;
  }
`;