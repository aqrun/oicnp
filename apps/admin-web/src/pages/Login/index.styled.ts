import styled from 'styled-components';
import { CLASS_PREFIX } from '~/constants';

export const Container = styled.div`
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
  background: #f1f1f1;

  h2 {
    text-align: center;
  }

  .${CLASS_PREFIX}-login-form {
    width: 400px;
    background: white;
    padding: 24px 30px;
    border-radius: 8px;
    box-shadow: 0 0 8px rgba(0,0,0,.1);
  }

  .${CLASS_PREFIX}-btn-login {
    width: 100%;
  }
`;