import { useState, useEffect } from 'react';
import { Form } from 'antd';
import { Modal } from '@/components';
import MenuForm from '../MenuForm';
import { useCreateStore } from './useCreateStore';
import { useMemoizedFn } from 'ahooks';
import CreateSuccess from './CreateSuccess';
import { useListStore } from '../MenuList/useListStore';
import {
  usePermissionTree,
  usePermissionTreeStore,
} from '@/components/PermissionTree'
import {
  MenuModel,
  DescribeCreateMenu,
  DescribeCreateMenuRequestParams,
  DescribeMenuPermissions,
  DescribeMenuPermissionsRequestParams,
} from '@/services';

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

  const [form] = Form.useForm<MenuModel>();

  const {
    fetchPermissionTree,
  } = usePermissionTree();

  const handleOk = useMemoizedFn(async () => {
    setLoading(true);
    try {
      const values = await form.validateFields();

      const params: DescribeCreateMenuRequestParams = {
        vid: values?.vid,
        name: values?.name,
        weight: values?.weight ?? 0,
        remark: values?.remark,
        status: values?.status,
      };

      const res = await DescribeCreateMenu(params);

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
    <MenuForm
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
      title="创建菜单"
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
