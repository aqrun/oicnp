'use server';

import { createFetcher } from '../../fetcher';
import { DescribeConsoleConfigResponseData } from './types';

const DescribeConsoleConfig = createFetcher<
void,
DescribeConsoleConfigResponseData
>('/console-config');

export async function getConsoleConfig() {
  const res = await DescribeConsoleConfig();
  return res;
}