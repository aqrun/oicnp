'use client';

import styled from 'styled-components';

export const HeroContainer = styled.div`
  background-image: url('/images/heros/tools.jpg');
  height: 200px;
`;

export const ToolList = styled.div`
  .tool-item-widget {
    margin-bottom: 24px;

    @media (min-width: 64rem) {
      &:nth-child(even) {
        .item-inner {
          margin-left: 8px;
        }
      }
      &:nth-child(odd) {
        .item-inner {
          margin-right: 8px;
        }
    }
    }
  }
  .item-inner {
    display: flex;
    align-items: center;
    padding: 1.1rem 1rem;
    border-radius: 10px;
    border: 1px solid transparent;
    box-shadow: 0 2px 7px rgb(146 146 146 / 12%);

    &:hover {
      border-color: var(--primary);
    }
  }
  .item-logo-w {
    width: 50px;
    height: 50px;
    border-radius: 4px;
    margin-right: 16px;
    background: var(--color-very-light-purple);
    padding: 4px;

    img {
      width: 100%;
      height: 100%;
      max-width: 100%;
    }
  }
  .item-content-w {
    flex: 1;
    min-height: 70px;
    display: flex;
    flex-direction: column;
    justify-content: center;

    .item-name {
      font-size: 16px;
      font-weight: 500;
      color: var(--text-color);
    }
    .item-description {
      font-size: 14px;
      color: var(--text-color-description);
      margin-top: 4px;
    }
  }
  .item-logo-text {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
    font-size: 30px;
    font-weight: 500;
    color: var(--text-color-description);
  }
`;