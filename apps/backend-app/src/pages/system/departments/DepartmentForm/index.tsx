import { ReactElement } from 'react';
import { Button, Form, Input, FormInstance } from 'antd';
import type { FormProps } from 'antd';
import { DepartmentModel } from '@repo/apis';
import { Container } from './index.styled';
type FieldType = DepartmentModel;

export interface DepartmentFormProps {
  onFinish?: FormProps<FieldType>['onFinish'];
  isEdit?: boolean;
  department?: DepartmentModel;
  loading?: boolean;
  showSubmit?: boolean;
  form?: FormInstance<DepartmentModel>;
  disabled?: boolean;
}

export default function DepartmentForm({
  onFinish,
  loading,
  isEdit,
  department,
  showSubmit,
  form,
  disabled,
}: DepartmentFormProps): ReactElement {
  const initialValues: FieldType = {
    name: '',
    pid: 0,
    vid: '',
    weight: 0,
    leader: '',
    phone: '',
    email: '',
    status: '',
    ...(department || {}),
  };

  return (
    <Container>
      <Form
        name="basic"
        initialValues={initialValues}
        onFinish={onFinish}
        autoComplete="off"
        layout="vertical"
        form={form}
        disabled={disabled}
      >
        <Form.Item<FieldType>
          label="名称"
          name="name"
          rules={[{ required: true, message: '请输入名称！' }]}
        >
          <Input />
        </Form.Item>
        <Form.Item<FieldType>
          label="VID"
          name="vid"
          rules={[{ required: true, message: '请输入VID！' }]}
        >
          <Input />
        </Form.Item>
        <Form.Item<FieldType>
          label="权重"
          name="weight"
          rules={[{ required: true, message: '请输入权重！' }]}
        >
          <Input />
        </Form.Item>
        <Form.Item<FieldType>
          label="负责人"
          name="leader"
          rules={[{ required: true, message: '请输入负责人！' }]}
        >
          <Input />
        </Form.Item>
        <Form.Item<FieldType>
          label="电话"
          name="phone"
          rules={[{ required: true, message: '请输入电话！' }]}
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
          label="状态"
          name="status"
          rules={[{ required: true, message: '请输入状态！' }]}
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
    </Container>
  );
}