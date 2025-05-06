import { useState, useEffect } from 'react';
import { Form, Descriptions, Spin } from 'antd';
import { Modal } from '@/components';
import { useViewStore } from './useViewStore';
import { useMemoizedFn } from 'ahooks';
import useDescriptions from './useDescriptions';
import {
  usePermissionTree,
} from '@/components/PermissionTree';
import {
  RoleModel,
  DescribeMenuDetail,
  DescribeMenuDetailRequestParams,
  DescribeMenuPermissions,
  DescribeMenuPermissionsRequestParams,
} from '@/services';

/**
 * 查看弹框
 */
export default function ViewModal() {
  const visible = useViewStore(state => state.visible);
  const menuId = useViewStore(state => state.menuId);
  const setState = useViewStore(state => state.setState);

  const [loading, setLoading] = useState(false);
  const [items] = useDescriptions();

  const [form] = Form.useForm<RoleModel>();

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
    const res = await DescribeMenuDetail(params);
    
    return res;
  });

  const fetchMenuPermissions = useMemoizedFn(async () => {
    const params: DescribeMenuPermissionsRequestParams = {
      id: menuId || 0,
    };
    const res = await DescribeMenuPermissions(params);
    return res;
  });

  const fetchInitialData = useMemoizedFn(async () => {
    setLoading(true);
    const requests: Array<Promise<any>> = [
      fetchPermissionTree(),
      fetchMenuPermissions(),
      fetchMenu(),
    ];
    const allRes = await Promise.all(requests);

    setState({
      menu: allRes?.[2]?.menu,
      menuPermissions: allRes?.[1]?.permissions,
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
