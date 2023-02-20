import { takeLatest, put, delay } from 'redux-saga/effects';
import { incrementWith } from '../reducers/counterSlice';

export function* incrementSyncWorker(action) {
  console.log('counter-worker');
  // yield delay(300);
  yield put(incrementWith(3));
  console.log('counter-worker-end');
}

export function* counterWatcher() {
  console.log('count-watcher');
  yield takeLatest('incrementSync', incrementSyncWorker);
}