'use client';

import styled from 'styled-components';

export const Container = styled.div`
  a {
    color: var(--ant-color-primary, #1677ff);

    &:hover {
      color: #85b1f0;
    }
  }
  .oic-divider {
    margin: 0;
  }
`;

export const LinkButtonWrapper = styled.a`
  
`;

export const DropdownItemWrapper = styled.div`
  color: var(--ant-color-primary, #1677ff);

  &.oic-danger {
    color: var(--ant-color-danger, #ff4d4f);

    a {
      &:hover {
        color: white;
      }
    }
  }

  a {
    padding: 5px 40px;
    display: block;
  }
`;