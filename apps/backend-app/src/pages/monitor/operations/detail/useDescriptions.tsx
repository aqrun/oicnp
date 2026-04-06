import { DescriptionsProps } from 'antd';
import { useViewStore } from './useViewStore';
import { formatDate } from '#src/utils';

export default function useDescriptions() {
  const operationLog = useViewStore(state => state.operationLog);

  const items: DescriptionsProps['items'] = [
    {
      key: 'id',
      label: 'ID',
      children: operationLog?.id,
    },
    {
      key: 'title',
      label: '标题',
      children: operationLog?.title,
    },
    {
      key: 'businessType',
      label: '业务类型',
      children: operationLog?.businessType,
    },
    {
      key: 'method',
      label: '方法',
      children: operationLog?.method,
    },
    {
      key: 'requestMethod',
      label: '请求方式',
      children: operationLog?.requestMethod,
    },
    {
      key: 'operatorType',
      label: '操作类型',
      children: operationLog?.operatorType,
    },
    {
      key: 'name',
      label: '操作人',
      children: operationLog?.name,
    },
    
    {
      key: 'departmentName',
      label: '部门名称',
      children: operationLog?.departmentName,
    },
    
    {
      key: 'ip',
      label: 'IP',
      children: operationLog?.ip,
    },
    
    {
      key: 'url',
      label: '请求URL',
      children: operationLog?.url,
    },
    
    {
      key: 'location',
      label: '位置',
      children: operationLog?.location,
    },
    
    {
      key: 'param',
      label: '参数',
      children: operationLog?.param,
    },
    {
      key: 'pathParam',
      label: '路径参数',
      children: operationLog?.pathParam,
    },
    {
      key: 'status',
      label: '状态',
      children: operationLog?.status,
    },
    
    {
      key: 'errorMessage',
      label: '错误信息',
      children: operationLog?.errorMessage,
    },
    
    {
      key: 'duration',
      label: '耗时',
      children: operationLog?.duration,
    },
    {
      key: 'createdAt',
      label: '创建时间',
      children: operationLog?.createdAt ? formatDate(operationLog?.createdAt) : '-',
    },
    {
      key: 'id-empty',
      label: '',
      children: '',
      span: 24,
    },
    {
      key: 'jsonResult',
      label: 'JSON结果',
      span: 24,
      children: operationLog?.jsonResult,
    },
  ];

  return [items];
}
