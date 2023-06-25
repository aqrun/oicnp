
export const IS_CLIENT = typeof window !== 'undefined';
export const WIN = (IS_CLIENT ? window : undefined) as Window;
export const IS_DEV = process.env.NODE_EVN === 'development'
  || (WIN?.location && WIN?.location?.href.indexOf('localhost') > 0);

export const API_PORT = 8000;