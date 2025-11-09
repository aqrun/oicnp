import { useState, useEffect } from 'react';
import { Form, Descriptions, Spin } from 'antd';
import { Modal } from '@/components';
import { useViewStore } from './useViewStore';
import { useMemoizedFn } from 'ahooks';
import useDescriptions from './useDescriptions';
import {
  PositionModel,
  DescribePositionDetail,
  DescribePositionDetailRequestParams,
} from '@/services';

/**
 * 查看弹框
 */
export default function ViewModal() {
  const visible = useViewStore(state => state.visible);
  const positionId = useViewStore(state => state.positionId);
  const setState = useViewStore(state => state.setState);

  const [loading, setLoading] = useState(false);
  const [items] = useDescriptions();

  const [form] = Form.useForm<PositionModel>();

  const handleCancel = useMemoizedFn(() => {
    form.resetFields();
    setState({
      visible: false,
      position: undefined,
    });
  });

  const fetchNote = useMemoizedFn(async () => {
    const params: DescribePositionDetailRequestParams = {
      positionId: positionId,
    };
    const res = await DescribePositionDetail(params);

    return res;
  });

  const fetchInitialData = useMemoizedFn(async () => {
    setLoading(true);

    const res = await fetchNote();

    setState({
      position: res?.position,
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
      width={640}
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
