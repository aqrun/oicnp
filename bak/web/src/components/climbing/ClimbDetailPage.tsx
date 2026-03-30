import {
  Footer,
  Header,
  HeaderBg,
  SideBar,
} from '@/components/HomePage';

import { Aotai } from './Aotai';
import {
  MountainList,
} from './MountainList';
import {
  mountains,
} from './mountains';
import { OilCalculator } from './OilCalculator';
import { Tabs } from './Tabs';

export interface ClimbDetailPageProps {
  tab?: 'index' | 'aotai' | 'oil';
}

export const ClimbDetailPage: React.FC<ClimbDetailPageProps> = ({
  tab,
}) => {
  const validTab = tab || 'index';
  return (
    <main>
      <Header />
      <HeaderBg />

      <section className='bg-white'>
        <div className='layout py-12 flex flex-col lg:flex-row gap-8'>
          <div className='oic-layout-content flex flex-col w-[calc(100% - 22rem)]'>
            <Tabs tab={validTab} />

            {validTab === 'index' && (
              <MountainList
                mountains={mountains}
              />
            )}
            {validTab === 'aotai' && (
              <Aotai />
            )}
            {validTab === 'oil' && (
              <OilCalculator />
            )}

          </div>
          <div className='lg:w-80'>
            <SideBar />
          </div>
        </div>
      </section>

      <Footer />
    </main>
  );
};