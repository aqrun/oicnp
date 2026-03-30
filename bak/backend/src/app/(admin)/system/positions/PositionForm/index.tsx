'use client';

import { Button, Form, Input, FormInstance } from 'antd';
import type { FormProps } from 'antd';
import { PositionModel } from '@/services';
import { Container } from './index.styled';
type FieldType = PositionModel;

export interface PositionFormProps {
  onFinish?: FormProps<FieldType>['onFinish'];
  isEdit?: boolean;
  position?: PositionModel;
  loading?: boolean;
  showSubmit?: boolean;
  form?: FormInstance<PositionModel>;
  disabled?: boolean;
}

export default function PositionForm({
  onFinish,
  loading,
  isEdit,
  position,
  showSubmit,
  form,
  disabled,
}: PositionFormProps): JSX.Element {
  const initialValues: FieldType = {
    positionId: 0,
    vid: '',
    name: '',
    weight: 0,
    status: '',
    remark: '',
    createdBy: 0,
    updatedBy: 0,
    createdAt: '',
    updatedAt: '',
    deletedAt: '',
    ...(position || {}),
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
        >
          <Input />
        </Form.Item>
        <Form.Item<FieldType>
          label="状态"
          name="status"
        >
          <Input />
        </Form.Item>
        <Form.Item<FieldType>
          label="备注"
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
    </Container>
  );
}