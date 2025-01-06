import styled from 'styled-components';
import {
  LAYOUT_HEADER_HEIGHT,
  CLASS_PREFIX,
} from '~/constants';

// Container component
export const Container = styled.div`
  width: 100%;
  height: 100%;
  min-height: 100vh;
  background-color: #f5f5f5;
`;

// Header component
export const Header = styled.header`
  height: ${LAYOUT_HEADER_HEIGHT}px;
  background: #001529;
  display: flex;
  align-items: center;
  padding: 0 0 0 50px;
  justify-content: space-between;

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
  height: calc(100vh - ${LAYOUT_HEADER_HEIGHT}px);
`;

// MainContent component
export const MainContent = styled.div`
  display: flex;
  height: 100%;
  overflow-y: auto;
  flex: 1;
  flex-direction: column;
  padding: 0 1rem;

  .${CLASS_PREFIX}-layout-bread {
    margin: 16px 0;
  }
`;

// Side component
export const Side = styled.aside`
  background-color: #001529;
  width: 240px;
  height: 100%;

  .${CLASS_PREFIX}-side-menu {
    height: 100%;
    overflow: auto;
  }

  .${CLASS_PREFIX}-icon {
    margin-right: 8px;
  }
`;

// Content component
export const Content = styled.div`
  flex: 1;
  background: white;
  border-radius: 8px;
  padding: 24px;
`;

// Footer component
export const Footer = styled.footer`
  height: 64px;
  min-height: 64px;
  display: flex;
  align-items: center;
  justify-content: center;
`;
