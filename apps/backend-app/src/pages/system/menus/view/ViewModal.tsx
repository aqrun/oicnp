import { useState, useEffect } from 'react';
import { Form, Descriptions, Spin } from 'antd';
import { Modal } from '#src/components';
import { useViewStore } from './useViewStore';
import { useMemoizedFn } from 'ahooks';
import useDescriptions from './useDescriptions';
import {
  usePermissionTree,
} from '#src/components/PermissionTree';
import type {
  MenuModel,
  DescribeMenuDetailRequestParams,
  DescribeMenuPermissionsRequestParams,
  DescribeMenuDetailResponseData,
  DescribeMenuPermissionsResponseData,
} from '@repo/apis';
import { menuApis } from '#src/api';

/**
 * 查看弹框
 */
export default function ViewModal() {
  const visible = useViewStore(state => state.visible);
  const menuId = useViewStore(state => state.menuId);
  const setState = useViewStore(state => state.setState);

  const [loading, setLoading] = useState(false);
  const [items] = useDescriptions();

  const [form] = Form.useForm<MenuModel>();

  const {
    fetchPermissionTree,
  } = usePermissionTree();

  const handleCancel = useMemoizedFn(() => {
    form.resetFields();
    setState({
      visible: false,
      menu: undefined,
    });
  });

  const fetchMenu = useMemoizedFn(async () => {
    const params: DescribeMenuDetailRequestParams = {
      id: menuId,
    };
    const res = await menuApis.DescribeMenuDetail(params);
    
    return res;
  });

  const fetchMenuPermissions = useMemoizedFn(async () => {
    const params: DescribeMenuPermissionsRequestParams = {
      id: menuId || 0,
    };
    const res = await menuApis.DescribeMenuPermissions(params);
    return res;
  });

  const fetchInitialData = useMemoizedFn(async () => {
    setLoading(true);
    const requests: Array<Promise<Response>> = [
      fetchPermissionTree(),
      fetchMenuPermissions(),
      fetchMenu(),
    ] as unknown as Array<Promise<Response>>;
    const allRes = await Promise.all(requests);

    setState({
      menu: (allRes?.[2] as unknown as DescribeMenuDetailResponseData)?.menu,
      menuPermissions: (allRes?.[1] as unknown as DescribeMenuPermissionsResponseData)?.permissions,
    });
    setLoading(false);
  });

  useEffect(() => {
    if (visible) {
      fetchInitialData();
    }
  }, [visible]);

  return (
    <Modal
      title="查看菜单"
      open={visible}
      onCancel={handleCancel}
      confirmLoading={false}
      cancelText={'关闭'}
      destroyOnClose
      width={640}
      hasOk={false}
    >
      <Spin
        spinning={loading}
      >
        <Descriptions
          items={items}
          column={2}
        />
      </Spin>
    </Modal>
  );
}
