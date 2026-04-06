import { useState, useEffect } from 'react';
import { Form, Skeleton } from 'antd';
import { Modal } from '#src/components';
import RoleForm from '../RoleForm';
import { useEditStore } from './useEditStore';
import { useMemoizedFn } from 'ahooks';
import Success from './Success';
import { useListStore } from '../RoleList/useListStore';
import {
  usePermissionTree,
  usePermissionTreeStore,
} from '#src/components/PermissionTree';
import type {
  RoleModel,
  DescribeRoleDetailRequestParams,
  DescribeUpdateRoleRequestParams,
  DescribeRolePermissionsRequestParams,
  PermissionModel,
} from '@repo/apis';
import { roleApis } from '#src/api';

/**
 * 更新弹框
 */
export default function EditModal() {
  const visible = useEditStore(state => state.visible);
  const contentType = useEditStore(state => state.contentType);
  const roleId = useEditStore(state => state.roleId);
  const role = useEditStore(state => state.role);
  const rolePermissions = useEditStore(state => state.rolePermissions);
  const setState = useEditStore(state => state.setState);
  const setListState = useListStore(state => state.setState);
  const setPermissionTreeState = usePermissionTreeStore(state => state.setState);

  const [loading, setLoading] = useState(false);
  const [initLoading, setInitLoading] = useState(true);

  const [form] = Form.useForm<RoleModel>();

  const {
    fetchPermissionTree,
  } = usePermissionTree();

  const fetchRole = useMemoizedFn(async () => {
    const params: DescribeRoleDetailRequestParams = {
      roleId,
    };
    const res = await roleApis.DescribeRoleDetail(params);
    
    setState({
      role: res.role,
    });
    
    form.setFieldsValue({
      vid: res?.role?.vid,
      name: res?.role?.name,
      remark: res?.role?.remark,
      weight: res?.role?.weight,
      status: res?.role?.status,
    });
  });

  const fetchRolePermissions = useMemoizedFn(async () => {
    const params: DescribeRolePermissionsRequestParams = {
      roleId,
    };
    const res = await roleApis.DescribeRolePermissions(params);
    setState({
      rolePermissions: res?.permissions,
    });
    return res?.permissions;
  });

  const handleOk = useMemoizedFn(async () => {
    setLoading(true);
    try {
      const values = await form.validateFields();

      const params: DescribeUpdateRoleRequestParams = {
        roleId,
        vid: values?.vid,
        name: values?.name,
        weight: Number(values?.weight ?? 0),
        remark: values?.remark,
        status: values?.status,
        permissionIds: values?.permissionIds,
      };

      const res = await roleApis.DescribeUpdateRole(params);

      if (res) {
        setState({
          contentType: 'success',
        });
      }
    } catch(err) {
      console.log('ERR: ', err);
    }

    setLoading(false);
  });

  const handleCancel = useMemoizedFn(() => {
    if (contentType === 'success') {
      form.resetFields();
      setListState({
        refreshToken: Date.now().toString(),
      });
    }

    setState({
      visible: false,
      contentType: 'normal',
    });
  });

  const getContent = () => {
    if (initLoading) {
      return (
        <Skeleton active />
      );
    } else if (contentType === 'success') {
      return (
        <Success
          roleName={form.getFieldValue('name')}
        />
      );
    } else {
      return (
        <RoleForm
          form={form}
          loading={loading}
          role={role}
        />
      );
    }
  };
  const content = getContent();

  const fetchInitialData = useMemoizedFn(async () => {
    setInitLoading(true);
    const requests = [
      fetchPermissionTree(),
      fetchRole(),
      fetchRolePermissions(),
    ];
    const allRes = await Promise.all(requests);

    // 更新表单值
    const permissions = (allRes?.[2] || []) as Array<PermissionModel>;
    const ids = permissions?.map((item) => item.permissionId) as Array<React.Key>;
    form.setFieldValue('permissionIds', ids);
    setPermissionTreeState({
      checkedKeys: ids,
    });
    
    setInitLoading(false);
  });

  useEffect(() => {
    if (visible) {
      fetchInitialData();
    }
  }, [visible]);

  return (
    <Modal
      title="编辑角色"
      open={visible}
      onOk={handleOk}
      onCancel={handleCancel}
      confirmLoading={false}
      okText="更新"
      cancelText={contentType !== 'success' ? '取消' : '关闭'}
      destroyOnClose
      width={640}
      hasOk={contentType !== 'success'}
      okButtonProps={{
        loading,
      }}
    >
      {content}
    </Modal>
  );
}
