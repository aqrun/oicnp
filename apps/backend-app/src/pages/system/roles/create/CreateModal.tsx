import { useState, useEffect } from 'react';
import { Form } from 'antd';
import { Modal } from '#src/components';
import RoleForm from '../RoleForm';
import { useCreateStore } from './useCreateStore';
import { useMemoizedFn } from 'ahooks';
import CreateSuccess from './CreateSuccess';
import { useListStore } from '../RoleList/useListStore';
import {
  usePermissionTree,
  usePermissionTreeStore,
} from '#src/components/PermissionTree'
import type { RoleModel, DescribeCreateRoleRequestParams } from '@repo/apis';
import { roleApis } from '#src/api';

/**
 * 创建弹框
 */
export default function CreateModal() {
  const visible = useCreateStore(state => state.visible);
  const contentType = useCreateStore(state => state.contentType);
  const setState = useCreateStore(state => state.setState);
  const setListState = useListStore(state => state.setState);
  const setPermissionTreeState = usePermissionTreeStore(state => state.setState);

  const [loading, setLoading] = useState(false);

  const [form] = Form.useForm<RoleModel>();

  const {
    fetchPermissionTree,
  } = usePermissionTree();

  const handleOk = useMemoizedFn(async () => {
    setLoading(true);
    try {
      const values = await form.validateFields();

      const params: DescribeCreateRoleRequestParams = {
        vid: values?.vid,
        name: values?.name,
        weight: values?.weight ?? 0,
        remark: values?.remark,
        status: values?.status,
        permissionIds: values?.permissionIds,
      };

      const res = await roleApis.DescribeCreateRole(params);

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

  const fetchInitialData = useMemoizedFn(async () => {
    setPermissionTreeState({
      checkedKeys: undefined,
    });
    await fetchPermissionTree();
  });

  let content = (
    <RoleForm
      form={form}
      loading={loading}
    />
  );

  if (contentType === 'success') {
    content = (
      <CreateSuccess
        roleName={form.getFieldValue('name')}
      />
    );
  }

  useEffect(() => {
    if (visible) {
      fetchInitialData();
    }
  }, [visible]);

  return (
    <Modal
      title="创建角色"
      open={visible}
      onOk={handleOk}
      onCancel={handleCancel}
      confirmLoading={false}
      okText="创建"
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
