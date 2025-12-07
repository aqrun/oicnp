import dayjs from 'dayjs';

export function formatDate(date?: string) {
  if (!date) return date;
  return dayjs(date).format('YYYY年MM月DD日');
}
