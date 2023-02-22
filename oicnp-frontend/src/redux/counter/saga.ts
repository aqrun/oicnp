import { takeLatest, put, delay, call } from 'redux-saga/effects';
import { incrementAsync, incrementAsyncSucceed } from '.';
import { IncrementAsyncAction } from './types';

export function* incrementAsyncWorker(action: IncrementAsyncAction) {
  const { num } = action?.payload!;
  const { actionResolved } = action?.meta || {};
  console.log('counter-worker', action);
  yield delay(3000);
  yield put(incrementAsyncSucceed(num));
  console.log('counter-worker-end');

  if (typeof actionResolved === 'function') {
    yield call(actionResolved);
  }
}

export function* counterWatcher() {
  yield takeLatest(incrementAsync.type, incrementAsyncWorker);
}
