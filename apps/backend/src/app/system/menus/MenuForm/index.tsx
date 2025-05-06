'use client';

import { Button, Form, Input, FormInstance, Radio } from 'antd';
import type { FormProps } from 'antd';
import { MenuModel } from '@/services';
import PermissionSelect from './PermissionSelect';
import { useMemoizedFn } from 'ahooks';
import { Container } from './index.styled';

type FieldType = MenuModel;

export interface MenuFormProps {
  onFinish?: FormProps<FieldType>['onFinish'];
  isEdit?: boolean;
  menu?: MenuModel;
  loading?: boolean;
  showSubmit?: boolean;
  form?: FormInstance<MenuModel>;
  disabled?: boolean;
}

export default function MenuForm({
  onFinish,
  loading,
  isEdit,
  menu,
  showSubmit,
  form,
  disabled,
}: MenuFormProps): JSX.Element {

  const initialValues: FieldType = {
    id: menu?.id || 0,
    vid: '',
    name: '',
    remark: '',
    weight: 0,
    status: '1',
    ...(menu || {}),
  };

  const statusOptions = [
    { value: '1', label: '启用'},
    { value: '0', label: '停用'}
  ];

  const handleTreeCheck = useMemoizedFn((checkedKeys: Array<React.Key>, info: any) => {
    form?.setFieldsValue({
      permissionIds: checkedKeys as Array<number>,
    });
  });

  return (
    <Container>
      <Form
        name="basic"
        wrapperCol={{ span: 24 }}
        initialValues={initialValues}
        onFinish={onFinish}
        autoComplete="off"
        layout="vertical"
        form={form}
        disabled={disabled}
      >
        <Form.Item<FieldType>
          label="标识"
          name="vid"
          rules={[{ required: true, message: '请输入标识！' }]}
          wrapperCol={{ span: 10 }}
        >
          <Input />
        </Form.Item>
        <Form.Item<FieldType>
          label="名称"
          name="name"
          rules={[{ required: true, message: '请输入角色名称！' }]}
          wrapperCol={{ span: 10 }}
        >
          <Input />
        </Form.Item>
        <Form.Item<FieldType>
          label="排序"
          name="weight"
          wrapperCol={{ span: 6 }}
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
        <PermissionSelect />

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