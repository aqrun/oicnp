import styled from 'styled-components';

export const MdContainer = styled.div`
  border: 1px solid #f0f0f0;
  border-radius: 4px;
  padding: 24px;
  width: 100%;

  h2, h3,p, ul, li, p {
    margin: 0;
  }
`;

export const ModelContent = styled.div`
  .oic-post-content {
    max-height: 600px;
    overflow-y: auto;
    padding: 16px;
  }
`;