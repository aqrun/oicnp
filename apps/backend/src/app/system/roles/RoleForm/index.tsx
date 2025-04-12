'use client';

import { Button, Form, Input, Checkbox } from 'antd';
import type { FormProps } from 'antd';
import { RoleModel } from '@/services';

type FieldType = RoleModel;

export interface RoleFormProps {
  onFinish?: FormProps<FieldType>['onFinish'];
  isEdit?: boolean;
  role?: RoleModel;
  loading?: boolean;
  showSubmit?: boolean;
}

export default function RoleForm({
  onFinish,
  loading,
  isEdit,
  role,
  showSubmit,
}: RoleFormProps): JSX.Element {

  const initialValues = {
    ...(role || {}),
  };

  return (
    <div>
      <Form
        name="basic"
        wrapperCol={{ span: 10 }}
        initialValues={initialValues}
        onFinish={onFinish}
        autoComplete="off"
        layout="vertical"
      >
        <Form.Item<FieldType>
          label="标识"
          name="vid"
          rules={[{ required: true, message: '请输入标识！' }]}
        >
          <Input />
        </Form.Item>
        <Form.Item<FieldType>
          label="名称"
          name="name"
          rules={[{ required: true, message: '请输入角色名称！' }]}
        >
          <Input />
        </Form.Item>
        <Form.Item<FieldType>
          label="邮箱"
          name="remark"
          rules={[{ required: true, message: '请输入描述！' }]}
        >
          <Input />
        </Form.Item>
        <Form.Item<FieldType>
          name="status"
          valuePropName="checked"
        >
          <Checkbox>启用</Checkbox>
        </Form.Item>
        <Form.Item<FieldType>
          name="permissionIds"
        >
          权限列表
        </Form.Item>

        {showSubmit && (
          <Form.Item label={null}>
            <Button
              type="primary"
              htmlType="submit"
              loading={loading}
            >
              {isEdit ? '更新' : '创建'}
            </Button>
          </Form.Item>
        )}
      </Form>
    </div>
  );
}