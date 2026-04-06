import { useMemoizedFn } from 'ahooks';
import { commonApis } from '#src/api';

export function useFetchCaptcha() {
	const fetchCaptcha = useMemoizedFn(async () => {
		const res = await commonApis.DescribeCaptcha();
		return res;
	});

	return { fetchCaptcha };
}
