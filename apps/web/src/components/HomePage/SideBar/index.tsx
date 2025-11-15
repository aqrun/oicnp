import { Categories } from './Categories';
import { Tags } from './Tags';
import { Weather1 } from '../../weather';

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
    <aside>
      {hasWeather && <Weather1 />}
      {hasCategories && <Categories />}
      {hasTags && <Tags />}
    </aside>
  );
};
