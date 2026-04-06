import type { MessageInstance } from "antd/es/message/interface";
import type { HookAPI } from "antd/es/modal/useModal";
import { createContext, useContext } from "react";

import type { HashState } from "#src/types/app";

export interface GlobalState {
	hashState?: HashState;
	modal: HookAPI;
	message: MessageInstance;
}

const initialGlobalData: GlobalState = {
	modal: null as unknown as HookAPI,
	message: null as unknown as MessageInstance,
};

export const GlobalContext = createContext<GlobalState>(initialGlobalData);

export function useGlobalState(): GlobalState {
	return useContext(GlobalContext);
}
