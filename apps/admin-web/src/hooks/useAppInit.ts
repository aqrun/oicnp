import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import { DescribeMenuList } from '~/api';
import { useAppStore } from '~/stores';

/**
 * 初始数据加载
 * @returns [fetchInitData, loading]
 */
export function useAppInit() {
  const setState = useAppStore((state) => state.setState);
  const [loading, setLoading] = useState(false);

  const fetchInitData = useMemoizedFn(async () => {
    setLoading(true);
    const res = await DescribeMenuList();
    
    setState({
      menus: res?.menus || [],
    });
    setLoading(false);
  });

  return [fetchInitData, loading] as const;
}