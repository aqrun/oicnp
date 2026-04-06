import { useState, useEffect } from 'react';
import { Form, Skeleton } from 'antd';
import { Modal } from '#src/components';
import MenuForm from '../MenuForm';
import { useEditStore } from './useEditStore';
import { useMemoizedFn } from 'ahooks';
import Success from './Success';
import { useListStore } from '../MenuList/useListStore';
import {
  usePermissionTree,
} from '#src/components/PermissionTree';
import type {
  MenuModel,
  DescribeMenuDetailRequestParams,
  DescribeUpdateMenuRequestParams,
  DescribeMenuPermissionsRequestParams,
  PermissionModel,
} from '@repo/apis';
import { menuApis } from '#src/api';

/**
 * 更新弹框
 */
export default function EditModal() {
  const visible = useEditStore(state => state.visible);
  const contentType = useEditStore(state => state.contentType);
  const menuId = useEditStore(state => state.menuId);
  const menu = useEditStore(state => state.menu);
  const setState = useEditStore(state => state.setState);
  const setListState = useListStore(state => state.setState);

  const [loading, setLoading] = useState(false);
  const [initLoading, setInitLoading] = useState(true);

  const [form] = Form.useForm<MenuModel>();

  const {
    fetchPermissionTree,
  } = usePermissionTree();

  const fetchMenu = useMemoizedFn(async () => {
    const params: DescribeMenuDetailRequestParams = {
      id: menuId,
    };
    const res = await menuApis.DescribeMenuDetail(params);
    
    setState({
      menu: res.menu,
    });
    
    form.setFieldsValue({
      vid: res?.menu?.vid,
      name: res?.menu?.name,
      remark: res?.menu?.remark,
      weight: res?.menu?.weight,
      status: res?.menu?.status,
      icon: res?.menu?.icon,
    });
  });

  const fetchMenuPermissions = useMemoizedFn(async () => {
    const params: DescribeMenuPermissionsRequestParams = {
      id: menuId,
    };
    const res = await menuApis.DescribeMenuPermissions(params);
    setState({
      menuPermissions: res?.permissions,
    });
    return res?.permissions;
  });

  const handleOk = useMemoizedFn(async () => {
    setLoading(true);
    try {
      const values = await form.validateFields();

      const params: DescribeUpdateMenuRequestParams = {
        id: menuId,
        vid: values?.vid,
        name: values?.name,
        weight: Number(values?.weight ?? 0),
        remark: values?.remark,
        status: values?.status,
        icon: values?.icon,
        permissionIds: values?.permissionIds,
      };

      const res = await menuApis.DescribeUpdateMenu(params);

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
        <MenuForm
          form={form}
          loading={loading}
          menu={menu}
        />
      );
    }
  };
  const content = getContent();

  const fetchInitialData = useMemoizedFn(async () => {
    setInitLoading(true);
    const requests = [
      fetchPermissionTree(),
      fetchMenu(),
      fetchMenuPermissions(),
    ];
    const allRes = await Promise.all(requests);

    // 更新表单值
    const permissions = (allRes?.[2] || []) as Array<PermissionModel>;
    const ids = permissions?.map((item) => item.permissionId) as Array<React.Key>;
    form.setFieldValue('permissionIds', ids);

    setInitLoading(false);
  });

  useEffect(() => {
    if (visible) {
      fetchInitialData();
    }
  }, [visible]);

  return (
    <Modal
      title="编辑菜单"
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
