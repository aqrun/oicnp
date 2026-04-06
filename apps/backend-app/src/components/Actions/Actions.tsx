'use client';

import type { ReactElement } from "react";

import { Dropdown, Space, Divider, MenuProps } from 'antd';
import React from 'react';
import { Icon } from '#src/components';
import { spreadFragmentInChildren } from './utils';
import clsx from 'clsx';
import { Container, DropdownItemWrapper } from './index.styled';

export interface ActionsProps {
  threshold?: number;
}

export default function Actions({
  children,
  threshold = 2,
}: React.PropsWithChildren<ActionsProps> ): ReactElement {
  const list = spreadFragmentInChildren(children);

  const menuItems = list.slice(threshold).map((item) => {
    const node = item as React.ReactElement<{ danger?: boolean }>;
    const nodeProps = node.props;

    const menuItem: NonNullable<MenuProps["items"]>[number] = {
      key: node?.key || "",
      label: (
        <DropdownItemWrapper
          className={clsx({
            "oic-danger": nodeProps?.danger,
          })}
        >
          {node}
        </DropdownItemWrapper>
      ),
      danger: nodeProps?.danger,
    };
    return menuItem;
  });

  return (
    <Container>
      <Space
        split={<Divider type="vertical" className="oic-divider" />}
        size="small"
      >
        {list.slice(0, threshold)}
        {menuItems?.length > 0 && (
          <Dropdown
            menu={{
              items: menuItems,
            }}
            placement="bottomRight"
            overlayClassName="oic-actions-dropdown-overlay"
          >
            <a>
              <Icon
                icon="MoreOutlined"
              />
            </a>
          </Dropdown>
        )}
      </Space>
    </Container>
  );
}
