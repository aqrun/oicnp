'use client';

import { useEffect } from 'react';
import type { FormProps } from 'antd';
import {
  PageTitle,
} from '@/components';
import { Container } from './index.styled';
import { useMemoizedFn } from 'ahooks';
import { useRouter } from 'next/navigation';
import { r } from '@/utils';
import {
  DescribeCreateUser,
  DescribeCreateUserRequestParams,
  useFetchRoleList,
 } from '@/services';
 import { useFetchUser } from '@/hooks/apis';
 import UserForm from './UserForm';
 import { useCreateStore } from './useCreateStore';
 import { FieldType } from '../types';

export default function UserCreatePage() {
  const setCreateState = useCreateStore(state => state.setState);
  const router = useRouter();
  const {
    fetchUserByUid,
  } = useFetchUser();
  const {
    fetchRoleList,
  } = useFetchRoleList();

  const handleBack = useMemoizedFn(() => {
    router.push(r('/system/users'));
  });

  const onFinish: FormProps<FieldType>['onFinish'] = async (values) => {
    const params: DescribeCreateUserRequestParams = {
      ...values,
      // boolean 转 字符串
      status: values?.status ? '1' : '0',
      isAdmin: values?.isAdmin ? '1' : '0',
    };
    const uid = await DescribeCreateUser(params) as number;
    const user = await fetchUserByUid(uid);

    // 创建成功
    if (user?.uuid) {
      router.push(r(`/system/users/create/success?uuid=${user?.uuid}&nickname=${user?.nickname}`));
    }
  };

  const fetchPageData = useMemoizedFn(async () => {
    const res = await fetchRoleList();
    setCreateState({
      roleList: res?.roles || [],
    });
  });

  useEffect(() => {
    fetchPageData();
  }, []);

  return (
    <Container>
      <PageTitle
        title='创建用户'
        onBack={handleBack}
      />
      <UserForm
        onFinish={onFinish}
      />
    </Container>
  );
}
