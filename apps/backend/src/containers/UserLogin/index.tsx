'use client';

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';
import {
  LockOutlined,
  MobileOutlined,
  UserOutlined,
} from '@ant-design/icons';
import {
  LoginFormPage,
  ProFormCaptcha,
  ProFormCheckbox,
  ProFormText,
} from '@ant-design/pro-components';
import { Button, message, Tabs } from 'antd';
import { useMemoizedFn } from 'ahooks';
import { r } from '~/utils';
import { useAuthState } from '~/hooks';
import { Container } from './index.styled';

type LoginType = 'phone' | 'account';

export default function UserLogin (): JSX.Element {
  const router = useRouter();
  const [, setAuthState] = useAuthState();
  const [loginType, setLoginType] = useState<LoginType>('account');

  const formSubmitHandle = useMemoizedFn((formData: Record<string, string>) => {
    if (formData.username === 'admin') {
      setAuthState({
        user: {
          username: formData.username,
        },
      });
      router.push(r('/welcome'));
    }
  });

  return (
    <Container>
      <LoginFormPage
        activityConfig={{
          style: {
            boxShadow: '0px 0px 8px rgba(0, 0, 0, 0.2)',
            color: '#fff',
            borderRadius: 8,
            backgroundColor: '#1677FF',
          },
          title: '活动标题，可配置图片',
          subTitle: '活动介绍说明文字',
          action: (
            <Button
              className="oic-btn-check-activity"
              size="large"
            >
              去看看
            </Button>
          ),
        }}
        backgroundImageUrl="https://gw.alipayobjects.com/zos/rmsportal/FfdJeJRQWjEeGTpqgBKj.png"
        logo="https://github.githubassets.com/images/modules/logos_page/Octocat.png"
        onFinish={formSubmitHandle}
        subTitle="全球最大的代码托管平台"
        title="Github"
      >
        <Tabs
          activeKey={loginType}
          centered
          onChange={(activeKey) => {setLoginType(activeKey as LoginType)}}
        >
          <Tabs.TabPane key="account" tab="账号密码登录" />
          <Tabs.TabPane disabled key="phone" tab="手机号登录" />
        </Tabs>
        {loginType === 'account' && (
          <>
            <ProFormText
              fieldProps={{
                size: 'large',
                prefix: <UserOutlined className="prefixIcon" />,
              }}
              name="username"
              placeholder="用户名: admin or user"
              rules={[
                {
                  required: true,
                  message: '请输入用户名!',
                },
              ]}
            />
            <ProFormText.Password
              fieldProps={{
                size: 'large',
                prefix: <LockOutlined className="prefixIcon" />,
              }}
              name="password"
              placeholder="密码: ant.design"
              rules={[
                {
                  required: true,
                  message: '请输入密码！',
                },
              ]}
            />
          </>
        )}
        {loginType === 'phone' && (
          <>
            <ProFormText
              fieldProps={{
                size: 'large',
                prefix: <MobileOutlined className="prefixIcon" />,
              }}
              name="mobile"
              placeholder="手机号"
              rules={[
                {
                  required: true,
                  message: '请输入手机号！',
                },
                {
                  pattern: /^1\d{10}$/,
                  message: '手机号格式错误！',
                },
              ]}
            />
            <ProFormCaptcha
              captchaProps={{
                size: 'large',
              }}
              captchaTextRender={(timing, count) => {
                if (timing) {
                  return `${count} ${'获取验证码'}`;
                }
                return '获取验证码';
              }}
              fieldProps={{
                size: 'large',
                prefix: <LockOutlined className="prefixIcon" />,
              }}
              name="captcha"
              onGetCaptcha={async () => {
                await message.success('获取验证码成功！验证码为：1234');
              }}
              placeholder="请输入验证码"
              rules={[
                {
                  required: true,
                  message: '请输入验证码！',
                },
              ]}
            />
          </>
        )}
        <div className="oic-auto-login-line-w" >
          <ProFormCheckbox name="autoLogin" noStyle>
            自动登录
          </ProFormCheckbox>
          <a className="oic-btn-forget" href="http://">
            忘记密码
          </a>
        </div>
      </LoginFormPage>
    </Container>
  );
};
