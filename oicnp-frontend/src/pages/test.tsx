import { SITE } from '../constants';
import { QueryNodesResponseData } from '../typings';
import { Home as HomeBase } from '../containers';
import { queryNodes } from '../services';
import { GetServerSideProps } from 'next';
import { checkIsMobile } from '~/utils';
import { useSelector, useDispatch } from 'react-redux';
import { selectCount, increment, incrementAsync } from '~/redux/counter';


export interface TestPageProps {

}

const TestPage: React.FC<TestPageProps> = (props) => {
  const dispatch = useDispatch();
  const count = useSelector(selectCount);

  return (
    <div
      className="max-w-7xl mx-auto py-11"
    >
      test page

      <div>
        counter: {count}
      </div>
      <div
        className="mr-2 cursor-pointer rounded-md bg-blue-600 text-white px-6 py-2 inline-block shadow-sm"
        onClick={() => {
          dispatch(increment());
        }}
      >
        Increment
      </div>
      <div
        className=" cursor-pointer rounded-md bg-blue-600 text-white px-6 py-2 inline-block shadow-sm"
        onClick={() => {
          dispatch(incrementAsync({ num: 3 }));
        }}
      >
        Increment Sync
      </div>
    </div>
  );
}

export default TestPage;