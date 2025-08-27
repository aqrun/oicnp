import { DescriptionsProps } from 'antd';
import { useViewStore } from './useViewStore';
import { formatDate } from '@/utils';

export default function useDescriptions() {
  const loginLog = useViewStore(state => state.loginLog);

  const items: DescriptionsProps['items'] = [
    {
      key: 'id',
      label: 'ID',
      children: loginLog?.id,
    },
    {
      key: 'loginAt',
      label: '登录时间',
      children: loginLog?.loginAt,
    },
    {
      key: 'loginName',
      label: '登录名',
      children: loginLog?.loginName,
    },
    {
      key: 'net',
      label: '网络',
      children: loginLog?.net,
    },
    {
      key: 'ip',
      label: 'IP',
      children: loginLog?.ip,
    },
    {
      key: 'location',
      label: '位置',
      children: loginLog?.location,
    },
    {
      key: 'browser',
      label: '浏览器',
      children: loginLog?.browser,
    },
    
    {
      key: 'os',
      label: '操作系统',
      children: loginLog?.os,
    },
    
    {
      key: 'device',
      label: '设备',
      children: loginLog?.device,
    },
    
    {
      key: 'status',
      label: '状态',
      children: loginLog?.status,
    },
    
    {
      key: 'message',
      label: '消息',
      children: loginLog?.message,
    },
    
    {
      key: 'module',
      label: '模块',
      children: loginLog?.module,
    },
    {
      key: 'status',
      label: '状态',
      children: loginLog?.status,
    },
    
    {
      key: 'loginAt',
      label: '登录时间',
      children: loginLog?.loginAt ? formatDate(loginLog?.loginAt) : '-',
    },
    {
      key: 'id-empty',
      label: '',
      children: '',
      span: 24,
    },
  ];

  return [items];
}
