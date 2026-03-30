import React from "react";
import { MenuItemProps } from 'antd';
import clsx from 'clsx';
import { LinkButtonWrapper } from './index.styled';

export interface LinkButtonProps extends MenuItemProps {

}

export default function LinkButton({
  children,
  ...props
}: React.PropsWithChildren<LinkButtonProps>): JSX.Element {
  const {
    danger,
    ...validProps
  } = props;
  
  return (
    <LinkButtonWrapper
      className={clsx({
        'oic-danger': danger,
      })}
      {...validProps as any}
    >
      {children}
    </LinkButtonWrapper>
  );
}