'use client';

import type { FormProps } from 'antd';
import { Button, Form, Input } from 'antd';
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

type FieldType = {
  username?: string;
  nickname?: string;
  password?: string;
  email?: string;
};

export default function UserCreatePage() {
  const router = useRouter();

  const handleBack = useMemoizedFn(() => {
    router.push(r('/system/users'));
  });

  const onFinish: FormProps<FieldType>['onFinish'] = async (values) => {
    console.log('Success:', values);
    const params: DescribeCreateUserRequestParams = {
      ...values,
    };
    const res = await DescribeCreateUser(params);

    // 创建成功
    if (res?.uuid) {
      router.push(r(`/system/users/create/success?uuid=${res?.uuid}&nickname=${res?.nickname}`));
    }
  };
  
  const onFinishFailed: FormProps<FieldType>['onFinishFailed'] = (errorInfo) => {
    console.log('Failed:', errorInfo);
  };

  return (
    <Container>
      <PageTitle
        title='创建用户'
        onBack={handleBack}
      />
      <Form
        name="basic"
        wrapperCol={{ span: 10 }}
        initialValues={{ remember: true }}
        onFinish={onFinish}
        onFinishFailed={onFinishFailed}
        autoComplete="off"
        layout="vertical"
      >
        <Form.Item<FieldType>
          label="用户名"
          name="username"
          rules={[{ required: true, message: '请输入用户名！' }]}
        >
          <Input />
        </Form.Item>
        <Form.Item<FieldType>
          label="昵称"
          name="nickname"
          rules={[{ required: true, message: '请输入用户名！' }]}
        >
          <Input />
        </Form.Item>
        <Form.Item<FieldType>
          label="邮箱"
          name="email"
          rules={[{ required: true, message: '请输入邮箱！' }]}
        >
          <Input />
        </Form.Item>
        <Form.Item<FieldType>
          label="密码"
          name="password"
          rules={[{ required: true, message: '请输入密码' }]}
        >
          <Input.Password />
        </Form.Item>

        {/* <Form.Item<FieldType> name="remember" valuePropName="checked" label={null}>
          <Checkbox>Remember me</Checkbox>
        </Form.Item> */}

        <Form.Item label={null}>
          <Button type="primary" htmlType="submit">
            创建
          </Button>
        </Form.Item>
      </Form>
    </Container>
  );
}
