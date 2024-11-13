import dayjs from 'dayjs';

/**
 * 日期格式化
 */
export function dateFormat(dateStr: string): string {
  const res = dayjs(dateStr).format('YYYY年MM月DD日 HH:mm:ss');
  return res;
}