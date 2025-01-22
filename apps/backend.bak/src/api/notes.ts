import { api } from '~/utils';
import type { NoteItem } from '~/types/notes';

export interface FetchNoteListRequestParams {

}

export interface FetchNoteListResponseData {
  page?: number;
  page_size?: number;
  total?: number;
  data?: NoteItem[];
}

export const fetchNoteList = api<
FetchNoteListResponseData,
FetchNoteListRequestParams
>('note/list');