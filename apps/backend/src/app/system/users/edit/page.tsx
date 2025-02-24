'use client';

import { useEffect } from 'react';
import { Skeleton } from 'antd';
import type { FormProps } from 'antd';
import {
  PageTitle,
} from '@/components';
import { Container } from './index.styled';
import { useMemoizedFn } from 'ahooks';
import { useRouter } from 'next/navigation';
import { r } from '@/utils';
import UserForm from '../create/UserForm';
import { useQueryUser } from './useQueryUser';
import useModal from '@/hooks/useModal';
import {
  DescribeUpdateUser,
  DescribeUpdateUserRequestParams,
 } from '@/services';
 import { FieldType } from '../types';

export default function UserCreatePage() {
  const router = useRouter();
  const {
    showError,
  } = useModal();

  const {
    data: user,
    loading,
  } = useQueryUser();

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

  const isLoading = !user || loading;

  useEffect(() => {
    if (!loading && !user?.uid) {
      showError({
        title: '用户不存在',
        content: '即将返回列表页',
        okText: '前往',
        onOk: () => {
          router.push(r('/system/users'));
        },
      });
    }
  }, [user, loading]);

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
        />
      )}
    </Container>
  );
}
