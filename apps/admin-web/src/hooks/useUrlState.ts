import { useLocation } from 'react-router';
import { useMemoizedFn } from 'ahooks';
import { useAppStore } from '~/stores';
import { getBaseUri, UrlState } from '~/utils';

export function useUrlState() {
  const menus = useAppStore((state) => state.menus);
  const location = useLocation();

  const getUrlState = useMemoizedFn((locationPathname: string) => {
    const baseUri = getBaseUri();
    const pathname = locationPathname?.replace(baseUri, '') || '/';

    const urlState = new UrlState(pathname, menus);
    return urlState;
  });

  const urlState = getUrlState(location?.pathname);

  return {
    menus,
    urlState,
    getUrlState,
  };
}
