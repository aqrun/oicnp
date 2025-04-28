'use client';

import { Button, Form, Input, FormInstance, Radio } from 'antd';
import type { FormProps } from 'antd';
import { MenuModel } from '@/services';
import { PermissionTree } from '@/components/PermissionTree';
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

  const handleTreeCheck = useMemoizedFn((checkedKeys: Array<string>, info: any) => {
    // form?.setFieldsValue({
    //   permissionIds: checkedKeys,
    // });
  });

  return (
    <Container>
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
        {/* <Form.Item<FieldType>
          name="permissionIds"
          label="权限列表"
          className="oic-permission-tree-form-item"
          wrapperCol={{ span: 24 }}
        >
          <PermissionTree
            onCheckChange={handleTreeCheck as any}
          />
        </Form.Item> */}

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