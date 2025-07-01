'use client';

import { useState, useEffect } from 'react';
import { Form, Input, Button, Checkbox } from 'antd';
import { CLASS_PREFIX } from '@/constants';
import cls from 'clsx';
import { loginAction } from './loginAction';
import { useFetchCaptcha, AuthCaptcha } from '@/services';
import { useMemoizedFn } from 'ahooks';
import Image from 'next/image';
import { Container } from './index.styled';

export interface FormValues {
  email: string;
  password: string;
  remember?: boolean;
  captcha?: string;
}

export default function Login() {
  const [form] = Form.useForm<FormValues>();

  const [errorInfo, setErrorInfo] = useState('');
  const [captchaRes, setCaptchaRes] = useState<AuthCaptcha | null>(null);
  const [loading, setLoading] = useState(false);

  const { fetchCaptcha } = useFetchCaptcha();
  
  /**
   * 刷新验证码
   */
  const refreshCaptcha = useMemoizedFn(async () => {
    const res = await fetchCaptcha();
    setCaptchaRes(res?.captcha);
  });

  const handleSubmit = useMemoizedFn(async () => {
    setLoading(true);
    setErrorInfo('');
    const values = form.getFieldsValue();

    const res = await loginAction({
      email: values?.email,
      password: values?.password,
      remember: Boolean(values?.remember),
      captchaId: captchaRes?.id,
      captcha: values?.captcha,
    });
    
    const code = res?.code || '200';

    if (code !== '200') {
      setErrorInfo(res?.message || '用户名或密码不正确');
      refreshCaptcha();
    }
    setLoading(false);
  });

  useEffect(() => {
    refreshCaptcha();
  }, []);
  
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
        <h2 className="text-2xl mb-4 text-slate-600 font-bold">OICNP ADMIN</h2>
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
        <div className="flex items-start mb-4">
          <Form.Item
            name="captcha"
            rules={[
              {
                required: true,
                message: '请输入正确的验证码',
              },
            ]}
            className="m-0 flex-1"
          >
            <Input
              placeholder="验证码"
            />
          </Form.Item>
          <div
            className="ml-2 cursor-pointer rounded-sm overflow-hidden"
            onClick={refreshCaptcha}
          >
            {Boolean(captchaRes?.img) && (
              <Image
                src={captchaRes?.img || 'img'}
                alt="captcha"
                width={100}
                height={30}
              />
            )}
          </div>
        </div>
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
            loading={loading}
          >
            登录
          </Button>
        </Form.Item>
      </Form>
    </Container>
  );
}
