import { useState, useEffect } from 'react';
import { Form, Descriptions, Spin } from 'antd';
import { Modal } from '@/components';
import { useViewStore } from './useViewStore';
import { useMemoizedFn } from 'ahooks';
import useDescriptions from './useDescriptions';
import {
  DepartmentModel,
  DescribeDepartmentDetail,
  DescribeDepartmentDetailRequestParams,
} from '@/services';

/**
 * 查看弹框
 */
export default function ViewModal() {
  const visible = useViewStore(state => state.visible);
  const departmentId = useViewStore(state => state.departmentId);
  const setState = useViewStore(state => state.setState);

  const [loading, setLoading] = useState(false);
  const [items] = useDescriptions();

  const [form] = Form.useForm<DepartmentModel>();

  const handleCancel = useMemoizedFn(() => {
    form.resetFields();
    setState({
      visible: false,
      department: undefined,
    });
  });

  const fetchNote = useMemoizedFn(async () => {
    const params: DescribeDepartmentDetailRequestParams = {
      id: departmentId,
    };
    const res = await DescribeDepartmentDetail(params);

    return res;
  });

  const fetchInitialData = useMemoizedFn(async () => {
    setLoading(true);

    const res = await fetchNote();

    setState({
      department: res?.department,
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
      title="查看部门"
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
