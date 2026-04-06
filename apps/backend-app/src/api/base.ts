import { createService } from '#src/utils/request';
import {
  createCommonApis,
  createCacheApis,
  createCategoryApis,
  createCronApis,
  createDepartmentApis,
  createFileApis,
  createLoginLogApis,
  createMenuApis,
  createNodeApis,
  createNoteApis,
  createOnlineApis,
  createOperationLogApis,
  createPermissionApis,
  createPoetryApis,
  createPositionApis,
  createRoleApis,
  createTagApis,
  createUserApis,
} from '@repo/apis';

export const commonApis = createCommonApis(createService);
export const cacheApis = createCacheApis(createService);
export const categoryApis = createCategoryApis(createService);
export const cronApis = createCronApis(createService);
export const departmentApis = createDepartmentApis(createService);
export const fileApis = createFileApis(createService);
export const loginLogApis = createLoginLogApis(createService);
export const menuApis = createMenuApis(createService);
export const nodeApis = createNodeApis(createService);
export const noteApis = createNoteApis(createService);
export const onlineApis = createOnlineApis(createService);
export const operationLogApis = createOperationLogApis(createService);
export const permissionApis = createPermissionApis(createService);
export const poetryApis = createPoetryApis(createService);
export const positionApis = createPositionApis(createService);
export const roleApis = createRoleApis(createService);
export const tagApis = createTagApis(createService);
export const userApis = createUserApis(createService);