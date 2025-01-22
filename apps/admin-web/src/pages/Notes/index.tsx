import { HashRouter, Routes, Route } from 'react-router';
import { r } from '~/utils';
import Layout from './layout';
import NoteList from './NoteList';
import NoteCreate from './NoteCreate';

export default function NotesPage(): JSX.Element {
  return (
    <HashRouter>
      <Routes>
        <Route path={r('/notes')} element={<Layout />}>
          <Route path={r('/notes/list')} element={<NoteList />} />
          <Route path={r('/notes/create')} element={<NoteCreate />} />
        </Route>
      </Routes>
    </HashRouter>
  );
}
