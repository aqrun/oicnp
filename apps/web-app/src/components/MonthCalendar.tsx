import { createRoot } from 'react-dom/client'
import SolarMonthCalendar from '@repo/calendar/SolarMonthCalendar';

export function renderMonthCalendar($container: HTMLElement) {
  createRoot($container).render(
    <SolarMonthCalendar />
  )
}