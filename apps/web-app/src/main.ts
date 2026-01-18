import '@vitejs/plugin-react-swc/preamble';
import { renderMonthCalendar } from './components/MonthCalendar';
import './style.css';
import './style.less';

interface ILx {
  renderMonthCalendar: typeof renderMonthCalendar;
}

function init() {
  const $s = document.getElementById('side-bar-calendar');
  if ($s) {
    renderMonthCalendar($s);
  }
}

(window as Window & typeof globalThis & { lx: ILx }).lx = {
  renderMonthCalendar
};

init();