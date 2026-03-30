import dayjs from 'dayjs';

export { useMemoizedFn as usePersistFn } from 'ahooks';

export function formatDate(date?: string) {
  if (!date) return date;
  return dayjs(date).format('YYYY-MM-DD');
}