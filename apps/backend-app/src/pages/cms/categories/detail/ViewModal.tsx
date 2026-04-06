import { useState, useEffect } from 'react';
import { Descriptions, Spin } from 'antd';
import { Modal } from '#src/components';
import { useViewStore } from './useViewStore';
import { useMemoizedFn } from 'ahooks';
import useDescriptions from './useDescriptions';
import type { DescribeCategoryDetailRequestParams } from "@repo/apis";
import { categoryApis } from "#src/api";

/**
 * 查看弹框
 */
export default function ViewModal() {
  const visible = useViewStore(state => state.visible);
  const catId = useViewStore(state => state.catId);
  const setState = useViewStore(state => state.setState);

  const [loading, setLoading] = useState(false);
  const [items] = useDescriptions();

  const handleCancel = useMemoizedFn(() => {
    setState({
      visible: false,
      category: undefined,
    });
  });

  const fetchTag = useMemoizedFn(async () => {
    const params: DescribeCategoryDetailRequestParams = {
      catId,
    };
    const res = await categoryApis.DescribeCategoryDetail(params);

    return res;
  });

  const fetchInitialData = useMemoizedFn(async () => {
    setLoading(true);
    const res = await fetchTag();

    setState({
      category: res?.category,
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
      title="查看分类"
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
