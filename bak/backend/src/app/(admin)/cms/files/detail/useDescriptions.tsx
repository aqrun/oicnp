import { DescriptionsProps } from 'antd';
import { useViewStore } from './useViewStore';
import { formatDate } from '@/utils';

export default function useDescriptions() {
  const file = useViewStore(state => state.file);

  const items: DescriptionsProps['items'] = [
    {
      key: 'fileId',
      label: 'ID',
      children: file?.fileId,
    },
    {
      key: 'filename',
      label: '文件名',
      children: file?.filename,
    },
    {
      key: 'uri',
      label: '文件路径',
      children: file?.uri,
    },
    {
      key: 'storage',
      label: '存储',
      children: file?.storage,
    },
    {
      key: 'mime',
      label: 'mime',
      children: file?.mime,
    },
    {
      key: 'status',
      label: '状态',
      children: file?.status,
    },
    {
      key: 'createdAt',
      label: '创建时间',
      children: formatDate(file?.createdAt || ''),
    },
  ];

  return [items];
}
