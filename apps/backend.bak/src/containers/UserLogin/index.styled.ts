import styled from 'styled-components';

export const Container = styled.div`
  background-color: white;
  height: calc(100vh);
  overflow: hidden;

  .ant-pro-form-login-page-container {
    // max-width: 50%;
  }
  .oic-auto-login-line-w {
    margin-block-end: 24px;
  }
  .oic-btn-forget {
    float: right;
  }

  .oic-btn-check-activity {
    border-radius: 20px;
    background: #fff;
    color: #1677FF;
    width: 120px;
  }

  @media screen and (max-width: 770px) {
    .ant-pro-form-login-page-container {
      max-width: 100%;
    }
  }
`;