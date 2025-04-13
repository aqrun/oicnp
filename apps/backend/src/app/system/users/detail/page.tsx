'use client';

import { useEffect, useState } from 'react';
import { useRouter, useSearchParams } from 'next/navigation';
import { useMemoizedFn } from 'ahooks';
import { r } from '@/utils';
import { Descriptions, Spin } from 'antd';
import type { DescriptionsProps } from 'antd';
import { useFetchUser } from '@/hooks/apis';
import {
  PageTitle,
} from '@/components';
import { UserModel } from '@/services';

/**
 * 用户信息
 */
export default function UserDetail(): JSX.Element {
  const router = useRouter();
  const searchParams = useSearchParams();
  const uid = searchParams?.get('uid');
  const {
    fetchUserByUid,
    loading,
  } = useFetchUser();
  const [user, setUser] = useState<UserModel | undefined>(undefined);

  const handleBack = useMemoizedFn(() => {
    router.push(r('/system/users'));
  });

  const items: DescriptionsProps['items'] = [
    {
      key: 'uuid',
      label: 'Uuid',
      children: user?.uuid,
    },
    {
      key: 'username',
      label: '用户名',
      children: user?.username,
    },
    {
      key: 'nickname',
      label: '昵称',
      children: user?.nickname,
    },
    {
      key: 'email',
      label: '邮箱',
      children: user?.email,
    },
    {
      key: 'status',
      label: '状态',
      children: user?.status,
    },
  ];

  const fetchInitData = useMemoizedFn(async () => {
    const res = await fetchUserByUid(Number(uid));
    setUser(res);
  });

  useEffect(() => {
    fetchInitData();
  }, []);

  return (
    <>
      <PageTitle
        title={`用户 ${'abc'}`}
        onBack={handleBack}
      />

      <Spin
        spinning={loading}
      >
        <Descriptions
          title="基本信息"
          items={items}
        />
      </Spin>
    </>
  );
}
