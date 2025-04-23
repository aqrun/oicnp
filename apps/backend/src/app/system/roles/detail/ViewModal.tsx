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
  DescribeRoleDetail,
  DescribeRoleDetailRequestParams,
  DescribeRolePermissionsRequestParams,
  DescribeRolePermissions,
} from '@/services';

/**
 * 查看弹框
 */
export default function ViewModal() {
  const visible = useViewStore(state => state.visible);
  const roleId = useViewStore(state => state.roleId);
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
      role: undefined,
    });
  });

  const fetchRolePermissions = useMemoizedFn(async () => {
    const params: DescribeRolePermissionsRequestParams = {
      roleId: roleId || 0,
    };
    const res = await DescribeRolePermissions(params);
    return res;
  });

  const fetchRole = useMemoizedFn(async () => {
    const params: DescribeRoleDetailRequestParams = {
      roleId,
    };
    const res = await DescribeRoleDetail(params);
    
    return res;
  });

  const fetchInitialData = useMemoizedFn(async () => {
    setLoading(true);
    const requests: Array<Promise<any>> = [
      fetchPermissionTree(),
      fetchRolePermissions(),
      fetchRole(),
    ];
    const allRes = await Promise.all(requests);

    setState({
      role: allRes?.[2]?.role,
      rolePermissions: allRes?.[1]?.permissions,
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
      title="查看角色"
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
