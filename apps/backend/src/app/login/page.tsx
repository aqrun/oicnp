'use client';

import { useState } from 'react';
import { Form, Input, Button, Checkbox } from 'antd';
import { CLASS_PREFIX } from '@/constants';
import cls from 'clsx';
import { loginAction } from './loginAction';
import { useMemoizedFn } from 'ahooks';
import { Container } from './index.styled';

export interface FormValues {
  email: string;
  password: string;
  remember?: boolean;
}

export default function Login() {
  const [form] = Form.useForm<FormValues>();

  const [errorInfo, setErrorInfo] = useState('');

  const handleSubmit = useMemoizedFn(async () => {
    setErrorInfo('');
    const values = form.getFieldsValue();

    const res = await loginAction({
      email: values?.email,
      password: values?.password,
      remember: Boolean(values?.remember),
    });
    
    const code = res?.code || '200';

    if (code !== '200') {
      setErrorInfo('用户名或密码不正确');
    }
  });
  
  return (
    <Container>
      <Form
        form={form}
        className={cls(`${CLASS_PREFIX}-login-form`)}
        initialValues={{
          email: 'guest@ab.com',
          password: '123456',
        }}
        onFinish={handleSubmit}
      >
        <h2 className="text-2xl mb-4 text-slate-800">OICNP ADMIN</h2>
        {Boolean(errorInfo) && (
          <div className="text-red-700 my-2">
            {errorInfo}
          </div>
        )}
        <Form.Item
          name="email"
          rules={[
            {
              required: true,
              message: '请输入正确的邮箱',
            },
          ]}        
        >
          <Input
            placeholder="邮箱"
          />
        </Form.Item>
        <Form.Item
          name="password"
          rules={[
            {
              required: true,
              message: '请输入正确的密码',
            },
          ]}        
        >
          <Input
            type="password"
            placeholder="密码"
          />
        </Form.Item>
        <Form.Item name="remember" valuePropName="checked">
          <Checkbox>
            记住我
          </Checkbox>
        </Form.Item>
        <Form.Item>
          <Button
            htmlType="submit"
            type="primary"
            className={cls(`${CLASS_PREFIX}-btn-login`)}
            // loading={login?.isPending}
          >
            登录
          </Button>
        </Form.Item>
      </Form>
    </Container>
  );
}
