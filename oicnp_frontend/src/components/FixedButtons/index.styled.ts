import styled from 'styled-components';

export const Container = styled.div`
  display: none;
  position: fixed;
  left: 50%;
  bottom: 100px;
  z-index: 1000;
  margin-left: 650px;
  width: 40px;
  font-size: 18px;

  &.show {
    display: block;
  }

  .oic-btn {
    box-shadow: 0 1px 3px rgb(26 26 26 / 10%);
    border-radius: 50%;
    text-align: center;
    width: 40px;
    height: 40px;
    line-height: 40px;
    cursor: pointer;

    .icon {
      font-size: 35px;
      color: #8c8c8c;
    }

    &:hover {
      box-shadow: 0 1px 3px rgb(26 26 26 / 50%);
      .icon {
        color: #666;
      }
    }
  }
`;