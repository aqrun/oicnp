'use client';

export async function nextTick () {
  return Promise.resolve(null);
}

/**
 * 安全函数调用
 */
export function callFn<Fn extends (...args: any[]) => any>(
  fn?: Fn,
  ...args: Parameters<Fn> | []
): ReturnType<Fn> {
  if (typeof fn === 'function') {
    return fn(...args);
  }

  return undefined as any;
}
