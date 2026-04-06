import { useState, useEffect } from 'react';
import { Descriptions, Spin } from 'antd';
import { Modal } from '#src/components';
import { useViewStore } from './useViewStore';
import { useMemoizedFn } from 'ahooks';
import useDescriptions from './useDescriptions';
import {   FileFilters } from "@repo/apis";
import { useFetchFile } from "#src/hooks/apis";

/**
 * 查看弹框
 */
export default function ViewModal() {
  const visible = useViewStore(state => state.visible);
  const fileId = useViewStore(state => state.fileId);
  const setState = useViewStore(state => state.setState);

  const [loading, setLoading] = useState(false);
  const [items] = useDescriptions();
  const {
    fetchFile,
  } = useFetchFile();

  const handleCancel = useMemoizedFn(() => {
    setState({
      visible: false,
      file: undefined,
    });
  });

  const fetchFileData = useMemoizedFn(async () => {
    const params: FileFilters = {
      fileId,
    };
    const res = await fetchFile(params);

    return res;
  });

  const fetchInitialData = useMemoizedFn(async () => {
    setLoading(true);
    const res = await fetchFileData();

    setState({
      file: res?.file,
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
      title="查看文件"
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
        />
      </Spin>
    </Modal>
  );
}
