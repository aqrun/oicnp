import { Categories } from './Categories';
import { Tags } from './Tags';
import { Weather1 } from '../../weather';
import SolarMonthCalendar from '@repo/calendar/SolarMonthCalendar';
import { RecommendBlogs } from './RecommendBlogs';

export interface SideBarProps {
  hasWeather?: boolean;
  hasCategories?: boolean;
  hasTags?: boolean;
}

export const SideBar: React.FC<SideBarProps> = ({
  hasWeather,
  hasCategories,
  hasTags,
}) => {
  return (
    <aside className="lg:w-80">
      <SolarMonthCalendar />
      {hasWeather && <Weather1 />}
      <RecommendBlogs />
      {hasCategories && <Categories />}
      {hasTags && <Tags />}
    </aside>
  );
};
