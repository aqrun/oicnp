import React, { useMemo } from 'react';
import ReactPaginate, {
  ReactPaginateProps,
} from 'react-paginate';
import {
  Container,
} from './index.styled';

export interface PaginatorProps extends ReactPaginateProps {
  _name?: string;
}

export const Paginator: React.FC<PaginatorProps> = (props) => {
  const options: ReactPaginateProps = useMemo(() => {
    return {
      previousLabel: <i className="iconfont icon-left" />,
      nextLabel: <i className="iconfont icon-right" />,
      ...props,
    };
  }, [props]);

  return (
    <Container className="oic-paginator-w">
      <ReactPaginate
        {...options}
      />
    </Container>
  );
}
