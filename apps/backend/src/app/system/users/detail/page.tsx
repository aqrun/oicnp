'use client';

import { useRouter } from 'next/navigation';
import { useMemoizedFn } from 'ahooks';
import { r } from '@/utils';
import { Descriptions } from 'antd';
import type { DescriptionsProps } from 'antd';
import {
  PageTitle,
} from '@/components';

/**
 * 用户信息
 */
export default function UserDetail(): JSX.Element {
  const router = useRouter();

  const handleBack = useMemoizedFn(() => {
    router.push(r('/system/users'));
  });

  const items: DescriptionsProps['items'] = [
    {
      key: 'uuid',
      label: 'Uuid',
      children: 'Zhou Maomao',
    },
    {
      key: 'username',
      label: '用户名',
      children: '1810000000',
    },
    {
      key: 'nickname',
      label: '昵称',
      children: 'Hangzhou, Zhejiang',
    },
    {
      key: 'email',
      label: '邮箱',
      children: 'empty@ab.com',
    },
    {
      key: 'status',
      label: '状态',
      children: '1',
    },
  ];

  return (
    <>
      <PageTitle
        title={`用户 ${'abc'}`}
        onBack={handleBack}
      />

      <Descriptions
        title="基本信息"
        items={items}
      />
    </>
  );
}
