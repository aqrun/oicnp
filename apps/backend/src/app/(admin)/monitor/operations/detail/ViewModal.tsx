import { useState, useEffect } from 'react';
import { Form, Descriptions, Spin } from 'antd';
import { Modal } from '@/components';
import { useViewStore } from './useViewStore';
import { useMemoizedFn } from 'ahooks';
import useDescriptions from './useDescriptions';
import {
  OperationLogModel,
  DescribeOperationLogDetail,
  DescribeOperationLogDetailRequestParams,
} from '@/services';

/**
 * 查看弹框
 */
export default function ViewModal() {
  const visible = useViewStore(state => state.visible);
  const operationLogId = useViewStore(state => state.operationLogId);
  const setState = useViewStore(state => state.setState);

  const [loading, setLoading] = useState(false);
  const [items] = useDescriptions();

  const [form] = Form.useForm<OperationLogModel>();

  const handleCancel = useMemoizedFn(() => {
    form.resetFields();
    setState({
      visible: false,
      operationLog: undefined,
    });
  });

  const fetchNote = useMemoizedFn(async () => {
    const params: DescribeOperationLogDetailRequestParams = {
      id: operationLogId,
    };
    const res = await DescribeOperationLogDetail(params);

    return res;
  });

  const fetchInitialData = useMemoizedFn(async () => {
    setLoading(true);

    const res = await fetchNote();

    setState({
      operationLog: res?.log,
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
      title="查看职位"
      open={visible}
      onCancel={handleCancel}
      confirmLoading={false}
      cancelText={'关闭'}
      destroyOnClose
      width={840}
      hasOk={false}
    >
      <Spin
        spinning={loading}
      >
        <Descriptions
          items={items}
          column={2}
          layout="vertical"
        />
      </Spin>
    </Modal>
  );
}
