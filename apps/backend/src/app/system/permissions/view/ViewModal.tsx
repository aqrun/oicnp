import { useState, useEffect } from 'react';
import { Form, Descriptions, Spin } from 'antd';
import { Modal } from '@/components';
import { useViewStore } from './useViewStore';
import { useMemoizedFn } from 'ahooks';
import useDescriptions from './useDescriptions';
import {
  PermissionModel,
  DescribePermissionDetail,
  DescribePermissionDetailRequestParams,
} from '@/services';

/**
 * 查看弹框
 */
export default function ViewModal() {
  const visible = useViewStore(state => state.visible);
  const permissionId = useViewStore(state => state.permissionId);
  const setState = useViewStore(state => state.setState);

  const [loading, setLoading] = useState(false);
  const [items] = useDescriptions();

  const [form] = Form.useForm<PermissionModel>();

  const handleCancel = useMemoizedFn(() => {
    form.resetFields();
    setState({
      visible: false,
      permission: undefined,
    });
  });

  const fetchInitialData = useMemoizedFn(async () => {
    setLoading(true);
    const params: DescribePermissionDetailRequestParams = {
      permissionId,
    };
    const res = await DescribePermissionDetail(params) as PermissionModel;
    const parentRes = await DescribePermissionDetail({
      permissionId: res?.pid,
    }) as PermissionModel;
    
    setState({
      permission: res,
      parentPermission: parentRes,
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
