import { useState, useEffect } from 'react';
import { Form, Skeleton } from 'antd';
import { Modal } from '@/components';
import RoleForm from '../RoleForm';
import { useEditStore } from './useEditStore';
import { useMemoizedFn } from 'ahooks';
import Success from './Success';
import { nextTick } from '@/utils/fn';
import { useListStore } from '../RoleList/useListStore';
import {
  RoleModel,
  DescribeRoleDetail,
  DescribeRoleDetailRequestParams,
  DescribeUpdateRole,
  DescribeUpdateRoleRequestParams,
} from '@/services';

/**
 * 更新弹框
 */
export default function EditModal() {
  const visible = useEditStore(state => state.visible);
  const contentType = useEditStore(state => state.contentType);
  const roleId = useEditStore(state => state.roleId);
  const role = useEditStore(state => state.role);
  const setState = useEditStore(state => state.setState);
  const setListState = useListStore(state => state.setState);

  const [loading, setLoading] = useState(false);
  const [initLoading, setInitLoading] = useState(true);

  const [form] = Form.useForm<RoleModel>();

  const fetchRole = useMemoizedFn(async () => {
    setInitLoading(true);
    const params: DescribeRoleDetailRequestParams = {
      roleId,
    };
    const res = await DescribeRoleDetail(params) as unknown as RoleModel;
    
    setState({
      role: res,
    });
    
    form.setFieldsValue({
      vid: res?.vid,
      name: res?.name,
      remark: res?.remark,
      weight: res?.weight,
      status: res?.status,
    });

    setInitLoading(false);
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

      const res = await DescribeUpdateRole(params);

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

  useEffect(() => {
    if (visible) {
      fetchRole();
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
