import { describe, expect, test } from 'vitest'
import { UrlState } from '~/utils';
import { menus } from './mocks/menus';

describe('UrlState 测试', () => {
  test('/ 首页解析', () => {
    const urlState = new UrlState('/', menus);
    expect(urlState.pathnames).toEqual(['main', 'dashboard']);
    expect(urlState.mainMenuKey).toBe('main');

    const urlState1 = new UrlState('', menus);
    expect(urlState1.pathnames).toEqual(['main', 'dashboard']);
    expect(urlState1.mainMenuKey).toBe('main');
  });

  test('/dashboard 仪表盘解析', () => {
    const urlState = new UrlState('/dashboard', menus);
    expect(urlState.pathnames).toEqual(['main', 'dashboard']);
  });

  test('/cms 内容管理解析', () => {
    const urlState = new UrlState('/cms', menus);
    expect(urlState.pathnames).toEqual(['cms', 'posts', 'list']);

    const urlState1 = new UrlState('/cms/categories', menus);
    expect(urlState1.pathnames).toEqual(['cms', 'categories']);
  });
});

