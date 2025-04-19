'use client';

import { Dropdown, Space, Divider, MenuProps } from 'antd';
import React from 'react';
import { Icon } from '@/components';
import { spreadFragmentInChildren } from './utils';
import clsx from 'clsx';
import { Container, DropdownItemWrapper } from './index.styled';

export interface ActionsProps {
  threshold?: number;
}

export default function Actions({
  children,
  threshold = 2,
}: React.PropsWithChildren<ActionsProps> ): JSX.Element {
  const list = spreadFragmentInChildren(children);

  const menuItems = list.slice(threshold).map((item) => {
    const node = item as React.ReactElement;

    const menuItem: NonNullable<MenuProps['items']>[number] = {
      key: node?.key || '',
      label: (
        <DropdownItemWrapper
          className={clsx({
            'oic-danger': node?.props?.danger,
          })}
        >
          {node}
        </DropdownItemWrapper>
      ),
      danger: node?.props?.danger,
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
      </Space>
    </Container>
  );
}
