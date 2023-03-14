import { useEffect } from 'react';
import { useMemoizedFn, useSize } from 'ahooks';
import { useRecoilState, useRecoilValue } from 'recoil';
import { isMobileState } from '~/atoms/isMobileState';
import { WIN } from '~/constants';

/**
 * @returns [checkIsMobile]
 */
export const useCheckIsMobile = () => {
  const [, setIsMobile] = useRecoilState(isMobileState);
  const size = useSize(WIN?.document.body);

  const checkIsMobile = useMemoizedFn(() => {
    if (WIN?.innerWidth < 768) {
      setIsMobile({
        value: true,
      });
    } else {
      setIsMobile({
        value: false,
      });
    }
  });

  useEffect(() => {
    checkIsMobile();
  }, [size, checkIsMobile]);

  return [checkIsMobile] as const;
};

export const useIsMobile = () => {
  const isMobile = useRecoilValue(isMobileState);
  return isMobile;
};