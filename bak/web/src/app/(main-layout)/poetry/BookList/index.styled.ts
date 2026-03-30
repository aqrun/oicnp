'use client';

import styled from 'styled-components';

export const HeroContainer = styled.div`
  background-color: var(--color-very-light-purple);
  background-image: url('/images/heros/books.png');
  height: 200px;
`;

export const BookListContainer = styled.div`
  .book-item {
    margin-bottom: 8px;
    font-size: 12px;
    
    .item-inner {
      display: flex;
      align-items: center;
      cursor: pointer;
      width: 200px;

      &:hover {
        color: var(--primary);
      }
    }
    .book-item-title {
      font-size: 14px;
    }
    .book-item-author {
      color: #999;
      font-size: 12px;
    }
  }
`;