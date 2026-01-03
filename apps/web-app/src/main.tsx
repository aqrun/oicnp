import './index.css';

function a() {
  return 1 + 2;
}

function b() {
  console.log('this is b');
}

function c() {
  console.log('this is c');
}

export interface ILx {
  a: () => number;
  b: () => void;
  c: () => void;
}

(window as Window & typeof globalThis & { lx: ILx }).lx = {
  a,
  b,
  c,
};