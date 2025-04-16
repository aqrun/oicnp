import { useState } from 'react';
import {
  DescribeUserDetail,
  DescribeUserDetailRequestParams,
} from '@/services';
import { useMemoizedFn } from 'ahooks';

export function useFetchUser() {
  const [loading, setLoading] = useState(false);
  
  const fetchUserByUid = useMemoizedFn(async (uid: number) => {
    setLoading(true);
    const params: DescribeUserDetailRequestParams = {
      uid,
    };
    const res = await DescribeUserDetail(params);
    setLoading(false);
    return res?.user;
  });

  return {
    loading,
    fetchUserByUid,
  };
}