import type { ButtonProps } from "antd";
import clsx from "clsx";
import type { ReactElement } from "react";
import React from "react";
import { LinkButtonWrapper } from "./index.styled";

export interface LinkButtonProps extends ButtonProps {

}

export default function LinkButton({
  children,
  ...props
}: React.PropsWithChildren<LinkButtonProps>): ReactElement {
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