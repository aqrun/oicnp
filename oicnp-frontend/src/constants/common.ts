
export const IS_CLIENT = typeof window !== 'undefined';
export const WIN = (IS_CLIENT ? window : undefined) as Window;