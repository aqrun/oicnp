import React from "react";
import { MenuItemProps } from 'antd';
import { LinkButtonWrapper } from './index.styled';

export interface LinkButtonProps extends MenuItemProps {

}

export default function LinkButton({
  children,
  ...props
}: React.PropsWithChildren<LinkButtonProps>): JSX.Element {
  return (
    <LinkButtonWrapper
      {...props as any}
    >
      {children}
    </LinkButtonWrapper>
  );
}