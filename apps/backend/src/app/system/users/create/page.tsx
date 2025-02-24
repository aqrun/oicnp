'use client';

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
 } from '@/services';
 import UserForm from './UserForm';
 import { FieldType } from '../types';

export default function UserCreatePage() {
  const router = useRouter();

  const handleBack = useMemoizedFn(() => {
    router.push(r('/system/users'));
  });

  const onFinish: FormProps<FieldType>['onFinish'] = async (values) => {
    console.log('Success:', values);
    const params: DescribeCreateUserRequestParams = {
      ...values,
      // boolean 转 字符串
      status: values?.status ? '1' : '0',
      isAdmin: values?.isAdmin ? '1' : '0',
    };
    const res = await DescribeCreateUser(params);

    // 创建成功
    if (res?.uuid) {
      router.push(r(`/system/users/create/success?uuid=${res?.uuid}&nickname=${res?.nickname}`));
    }
  };

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
