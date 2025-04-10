'use client';

import React from 'react';
import { Button, Form, Input, Checkbox } from 'antd';
import type { FormProps } from 'antd';
import {
  FieldType,
} from '../types';
import { DescribeUserDetailResponseData } from '@/services';
import {
  UserFormContainer,
} from './index.styled';

export interface UserFormProps {
  onFinish: FormProps<FieldType>['onFinish'];
  isEdit?: boolean;
  user?: DescribeUserDetailResponseData;
  loading?: boolean;
}

export default function UserForm({
  onFinish,
  isEdit,
  user,
  loading,
}: UserFormProps): JSX.Element {
  let initialValues: Partial<FieldType> = {
    remember: true,
  };

  if (isEdit) {
    initialValues = {
      username: user?.username,
      nickname: user?.nickname,
      email: user?.email,
      status: user?.status,
      isAdmin: user?.isAdmin,
    };
  }

  return (
    <UserFormContainer>
      <Form
        name="basic"
        wrapperCol={{ span: 10 }}
        initialValues={initialValues}
        onFinish={onFinish}
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
        {!isEdit && (
          <Form.Item<FieldType>
            label="密码"
            name="password"
            rules={[{ required: true, message: '请输入密码' }]}
          >
            <Input.Password />
          </Form.Item>
        )}
        <Form.Item<FieldType>
          name="status"
          valuePropName="checked"
        >
          <Checkbox>账号启用</Checkbox>
        </Form.Item>
        <Form.Item<FieldType>
          name="isAdmin"
          valuePropName="checked"
        >
          <Checkbox>超级管理员</Checkbox>
        </Form.Item>

        <Form.Item label={null}>
          <Button
            type="primary"
            htmlType="submit"
            loading={loading}
          >
            {isEdit ? '更新' : '创建'}
          </Button>
        </Form.Item>
      </Form>
    </UserFormContainer>
  );
}