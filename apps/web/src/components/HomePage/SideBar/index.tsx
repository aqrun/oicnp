import { Categories } from './Categories';
import { Tags } from './Tags';
import { Weather1 } from '../../weather';

export const SideBar = () => {
  return (
    <aside>
      <Weather1 />
      <Categories />
      <Tags />
    </aside>
  );
};
