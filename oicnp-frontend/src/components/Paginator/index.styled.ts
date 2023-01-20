import styled from 'styled-components';
import * as colors  from '../../styles/colors';

export const Container = styled.div`
  display: flex;
  justify-content: center;
  
  ul {
    display: flex;
  }
  li {
    border-radius: 2px;
    border: 1px solid #d9d9d9;
    margin-right: 8px;

    &.selected,
    &:hover {
      border-color: ${colors.BLUE_PRIMARY}
    }
  }
  a {
    display: block;
    height: 30px;
    line-height: 30px;
    width: 30px;
    text-align: center;

    &:hover {
      color: ${colors.BLUE_PRIMARY};
    }
  }
`;