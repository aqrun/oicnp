import { useState, useEffect } from 'react';
import { Form, Skeleton } from 'antd';
import { Modal } from '@/components';
import PermissionForm from '../PermissionForm';
import { useEditStore } from './useEditStore';
import { useMemoizedFn } from 'ahooks';
import Success from './Success';
import { useListStore } from '../PermissionList/useListStore';
import {
  PermissionModel,
  DescribePermissionDetail,
  DescribePermissionDetailRequestParams,
  DescribeUpdatePermission,
  DescribeUpdatePermissionRequestParams,
} from '@/services';

/**
 * 更新弹框
 */
export default function EditModal() {
  const visible = useEditStore(state => state.visible);
  const contentType = useEditStore(state => state.contentType);
  const permissionId = useEditStore(state => state.permissionId);
  const permission = useEditStore(state => state.permission);
  const setState = useEditStore(state => state.setState);
  const setListState = useListStore(state => state.setState);

  const [loading, setLoading] = useState(false);
  const [initLoading, setInitLoading] = useState(true);

  const [form] = Form.useForm<PermissionModel>();

  const fetchRole = useMemoizedFn(async () => {
    setInitLoading(true);
    const params: DescribePermissionDetailRequestParams = {
      permissionId,
    };
    const res = await DescribePermissionDetail(params) as unknown as PermissionModel;
    
    setState({
      permission: res,
    });
    
    form.setFieldsValue({
      vid: res?.vid,
      name: res?.name,
      remark: res?.remark,
      weight: res?.weight,
      status: res?.status,
      pid: res?.pid,
    });

    setInitLoading(false);
  });

  const handleOk = useMemoizedFn(async () => {
    setLoading(true);
    try {
      const values = await form.validateFields();

      const params: DescribeUpdatePermissionRequestParams = {
        permissionId,
        vid: values?.vid,
        pid: Number(values?.pid || 0),
        name: values?.name,
        weight: Number(values?.weight ?? 0),
        remark: values?.remark,
        status: values?.status,
      };

      const res = await DescribeUpdatePermission(params);

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
          name={form.getFieldValue('name')}
        />
      );
    } else {
      return (
        <PermissionForm
          form={form}
          loading={loading}
          permission={permission}
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
      title="编辑权限"
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
