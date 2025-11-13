import dayjs from 'dayjs';

export { useMemoizedFn as usePersistFn } from 'ahooks';

export function formatDate(date = new Date()) {
  return dayjs(date).format('YYYY-MM-DD');
}