'use client';

import { Button, Form, Input, FormInstance, Radio } from 'antd';
import type { FormProps } from 'antd';
import { PermissionModel } from '@/services';

type FieldType = PermissionModel;

export interface PermissionFormProps {
  onFinish?: FormProps<FieldType>['onFinish'];
  isEdit?: boolean;
  permission?: PermissionModel;
  loading?: boolean;
  showSubmit?: boolean;
  form?: FormInstance<PermissionModel>;
  disabled?: boolean;
}

export default function PermissionForm({
  onFinish,
  loading,
  isEdit,
  permission,
  showSubmit,
  form,
  disabled,
}: PermissionFormProps): JSX.Element {

  const initialValues: FieldType = {
    vid: '',
    name: '',
    remark: '',
    weight: 0,
    status: '1',
    ...(permission || {}),
  };

  const statusOptions = [
    { value: '1', label: '启用'},
    { value: '0', label: '停用'}
  ];

  return (
    <div>
      <Form
        name="basic"
        wrapperCol={{ span: 10 }}
        initialValues={initialValues}
        onFinish={onFinish}
        autoComplete="off"
        layout="vertical"
        form={form}
        disabled={disabled}
      >
        <Form.Item<FieldType>
          label="父级权限"
          name="pid"
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
          label="标识"
          name="vid"
          rules={[{ required: true, message: '请输入标识！' }]}
        >
          <Input />
        </Form.Item>
        <Form.Item<FieldType>
          label="排序"
          name="weight"
        >
          <Input />
        </Form.Item>
        <Form.Item<FieldType>
          label="状态"
          name="status"
        >
          <Radio.Group
            options={statusOptions}
          />
        </Form.Item>
        <Form.Item<FieldType>
          label="描述"
          name="remark"
        >
          <Input />
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
