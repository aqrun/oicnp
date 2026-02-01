import '@vitejs/plugin-react-swc/preamble';
import { renderMonthCalendar } from './components/MonthCalendar';
import hljs from 'highlight.js';

import 'highlight.js/styles/github-dark.css';
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

  hljs.highlightAll();

  // 如果使用 HTMX，需要在内容更新后重新高亮
  document.body.addEventListener('htmx:afterSwap', function() {
    hljs.highlightAll();
  });
}

(window as Window & typeof globalThis & { lx: ILx }).lx = {
  renderMonthCalendar
};

init();