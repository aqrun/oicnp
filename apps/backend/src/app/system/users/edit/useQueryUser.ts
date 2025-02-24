'use client';

import {
  DescribeUserDetail,
  DescribeUserDetailRequestParams,
} from '@/services';
import { useQuery } from '@tanstack/react-query';
import { useSearchParams } from 'next/navigation';

export function useQueryUser() {
  const searchParams = useSearchParams();
  const uid = searchParams?.get('uid');

  const { isFetching, data } = useQuery({
    queryKey: ['queryUser', uid],
    queryFn: async () => {
      const params: DescribeUserDetailRequestParams = {
        uid: Number(uid),
      };

      const res = await DescribeUserDetail(params);
      return res;
    }
  });

  return {
    data,
    loading: isFetching,
  };
}
