'use client';

import styled from 'styled-components';
import {
  LAYOUT_HEADER_HEIGHT,
  CLASS_PREFIX,
} from '@/constants';

// Container component
export const Container = styled.div`
  width: 100%;
  height: 100%;
  min-height: 100vh;
  background-color: #f0f4fb;
  display: flex;
  flex-direction: row;
`;

// Header component
export const Header = styled.header`
  height: ${LAYOUT_HEADER_HEIGHT}px;
  border-right: 1px solid hsl(var(--border));
  border-bottom: 1px solid hsl(var(--border));
  background: transparent;
  display: flex;
  align-items: center;
  padding: 0 0 0 24px;
  justify-content: center;

  .${CLASS_PREFIX}-logo {
    width: 120px;
    height: 32px;
    background: rgba(255, 255, 255, 0.3);
    border-radius: 4px;
    text-align: center;
  }
  .${CLASS_PREFIX}-header-main-menu {
    margin-left: 50px;

    .${CLASS_PREFIX}-icon {
      margin-right: 8px;
    }
  }
`;

// Main component
export const Main = styled.main`
  display: flex;
  height: 100vh;
  flex: 1;
`;

// MainContent component
export const MainContent = styled.div`
  display: flex;
  height: 100%;
  overflow-y: auto;
  flex: 1;
  flex-direction: column;
  padding: 0 24px;
  background: white;

  .${CLASS_PREFIX}-layout-bread {
    margin: 16px 0;
  }
`;

// Side component
export const Side = styled.aside`
  background-color: white;
  width: 256px;
  height: 100vh;
  display: flex;
  justify-content: space-between;
  flex-direction: column;

  .${CLASS_PREFIX}-side-menu {
    height: calc(100% - 64px - 64px);
    overflow: auto;
    background: transparent;
    color: #f4f4f5;
  }

  .${CLASS_PREFIX}-icon {
    margin-right: 8px;
  }

  .${CLASS_PREFIX}-layout-side-footer {
    height: 80px;
    display: flex;
    // align-items: center;
    // justify-content: center;
    background: white;
    // border-top: 1px solid hsl(var(--border));
    border-right: 1px solid hsl(var(--border));
  }
`;

// Content component
export const Content = styled.div`
  flex: 1;
`;

// Footer component
export const Footer = styled.footer`
  height: 64px;
  min-height: 64px;
  display: flex;
  align-items: center;
  justify-content: center;
`;
