import styled from 'styled-components';
import { Layout } from 'antd';

export const Container = styled(Layout.Header)`
  padding: 0 50px;
  display: flex;
  align-items: center;

  .logo {
    display: flex;
    align-items: center;
    width: 30px;
    height: 30px;
    margin-right: 20px;

    img {
      max-height: 100%;
    }
  }
`;

