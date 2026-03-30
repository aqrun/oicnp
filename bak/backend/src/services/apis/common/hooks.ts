import { useMemoizedFn } from 'ahooks';
import { DescribeCaptcha } from './client';

export function useFetchCaptcha() {
  const fetchCaptcha = useMemoizedFn(async () => {
    const res = await DescribeCaptcha();

    return res;
  });

  return { fetchCaptcha };
}