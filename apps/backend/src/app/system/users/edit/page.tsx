'use client';

import { useEffect, useState } from 'react';
import { Skeleton } from 'antd';
import type { FormProps } from 'antd';
import {
  PageTitle,
} from '@/components';
import { Container } from './index.styled';
import { useMemoizedFn } from 'ahooks';
import { useRouter, useSearchParams } from 'next/navigation';
import { r } from '@/utils';
import UserForm from '../create/UserForm';
import useModal from '@/hooks/useModal';
import {
  DescribeUpdateUser,
  DescribeUpdateUserRequestParams,
  useFetchUserRoles,
  useFetchUser,
  useFetchRoleList,
 } from '@/services';
 import { FieldType } from '../types';
 import { useEditStore } from './useEditStore';
 import { useCreateStore } from '../create/useCreateStore';

export default function UserCreatePage() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const uid = searchParams?.get('uid');

  const userRoles = useEditStore(state => state.userRoles);
  const user = useEditStore(state => state.user);
  const setEditState = useEditStore(state => state.setState);
  const setCreateState = useCreateStore(state => state.setState);

  const [loading, setLoading] = useState(false);

  const {
    showError,
  } = useModal();
  const {
    fetchUserRoles,
  } = useFetchUserRoles();
  const {
    fetchUser,
  } = useFetchUser();
  const {
    fetchRoleList,
  } = useFetchRoleList();

  const handleBack = useMemoizedFn(() => {
    router.push(r('/system/users'));
  });

  const onFinish: FormProps<FieldType>['onFinish'] = async (values) => {
    const params: DescribeUpdateUserRequestParams = {
      ...values,
      uid: user?.uid,
      // boolean 转 字符串
      status: values?.status ? '1' : '0',
      isAdmin: values?.isAdmin ? '1' : '0',
    };
    const res = await DescribeUpdateUser(params);
    console.log('Success---res:', res);
    // 编辑成功
    if (res) {
      router.push(r(`/system/users/edit/success?uid=${res}`));
    }
  };

  const fetchPageData = useMemoizedFn(async () => {
    setLoading(true);
    const userRes = await fetchUser({ uid: Number(uid) });
    const user = userRes?.user;
    const roleListRes = await fetchRoleList();
    const userRoleRes = await fetchUserRoles({ uid: user?.uid });

    if (!user?.uid) {
      showError({
        title: '用户不存在',
        content: '即将返回列表页',
        okText: '前往',
        onOk: () => {
          router.push(r('/system/users'));
        },
      });
      setLoading(false);
      return;
    }

    setCreateState({
      roleList: roleListRes?.roles,
    });
    setEditState({
      user,
      userRoles: userRoleRes?.roles,
    });
    setLoading(false);
  });

  const isLoading = !user || loading;

  useEffect(() => {
    fetchPageData();
  }, []);

  return (
    <Container>
      <PageTitle
        title='更新用户'
        onBack={handleBack}
      />
      {isLoading && (
        <Skeleton />
      )}
      {!isLoading && (
        <UserForm
          onFinish={onFinish}
          isEdit
          user={user}
          loading={loading}
          roleIds={userRoles?.map(item => item.roleId || 0)}
        />
      )}
    </Container>
  );
}
