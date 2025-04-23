'use client';

import { useMemo } from 'react';
import { Button, Form, Input, FormInstance, Radio } from 'antd';
import type { FormProps } from 'antd';
import { RoleModel, PermissionModel } from '@/services';
import { PermissionTree } from '@/components/PermissionTree';
import { useMemoizedFn } from 'ahooks';
import { Container } from './index.styled';

type FieldType = RoleModel;

export interface RoleFormProps {
  onFinish?: FormProps<FieldType>['onFinish'];
  isEdit?: boolean;
  role?: RoleModel;
  loading?: boolean;
  showSubmit?: boolean;
  form?: FormInstance<RoleModel>;
  disabled?: boolean;
  rolePermissions?: Array<PermissionModel>;
}

export default function RoleForm({
  onFinish,
  loading,
  isEdit,
  role,
  showSubmit,
  form,
  disabled,
  rolePermissions,
}: RoleFormProps): JSX.Element {

  const initialValues: FieldType = {
    vid: '',
    name: '',
    remark: '',
    weight: 0,
    status: '1',
    ...(role || {}),
  };

  const statusOptions = [
    { value: '1', label: '启用'},
    { value: '0', label: '停用'}
  ];

  const checkedPermissionsIds = useMemo(() => {
    return (rolePermissions || [])?.map((item) => item.permissionId) as Array<React.Key>;
  }, [rolePermissions]);

  const handleTreeCheck = useMemoizedFn((checkedKeys: Array<string>, info: any) => {
    form?.setFieldsValue({
      permissionIds: checkedKeys,
    });
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
        <Form.Item<FieldType>
          name="permissionIds"
          label="权限列表"
          className="oic-permission-tree-form-item"
          wrapperCol={{ span: 24 }}
        >
          <PermissionTree
            onCheck={handleTreeCheck as any}
            defaultCheckedKeys={checkedPermissionsIds}
          />
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