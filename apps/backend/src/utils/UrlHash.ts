import qs from 'qs';

export interface AddParamOptions {
  /**
   * 是否替换原有数据
   *
   * default: true
   */
  replace?: boolean;
}

/**
 * # URL hash 数据操作
 *
 * 解析hash片段标识符字符串及相关操作封装
 *
 * 如：
 * #/lowCode/configView?name=alex
 *
 * ## 功能
 *
 * 完整的URL格式：
 *
 * https://www.aliyun.com:8081/main/path?name=alex#/lowCode/test?mark=url-params-test
 *
 * * 协议(schema): 指定资源访问使用的协议，如 http/https/ftp等
 * * 主机名(host): 指定服务器域名或IP地址，如 www.aliyun.com
 * * 端口号(port): 指定连接到服务器具体端口号，如默认http 80, https 443，8081
 * * 路径(path): 指向服务器具体资源位置，如 `/main/path`
 * * 查询字符串(query): 可选，键值对，用于向服务器传递参数，如 `?name=alex`
 * * 片段标识符(fragment): 哈希参数，用于指定资源内部的一个特定部分，如 `#/lowCode/test?mark=url-params-test`
 *
 * 其中片段标识符部分，#号之后的内容可以自定义，不会触发浏览器刷新，
 * 本类 `URLHash` 主要用于哈希片段标识符部分内容解析、修改等操作
 *
 * ## 示例：
 *
 * ```typescript
 * const hashStr = '#/lowCode/configView?name=alex';
 * const urlHash = new URLHash(hashStr);
 * const newHashStr = urlHash.setPathName('/this/is/new/path')
 *    .addParams({
 *      age: 18,
 *    })
 *    .toString();
 * expect(newHashStr).toBe('#/this/is/new/path?name=alex&age=18');
 * ```
 */
export class URLHash {
  /**
   * URLSearchParams 格式的hash参数
   */
  params: URLSearchParams = new URLSearchParams();
  /**
   * hash参数字符串
   *
   * ?a=1&b=2
   */
  search = '';
  /**
   * hash 路径
   * /a/b/c
   */
  pathName = '';
  /**
   * hash全部数据
   */
  hash = '';
  /**
   * 解析参数options
   */
  _parseOptions: qs.IParseOptions = {};

  constructor(paramHash: string | Location, options: qs.IParseOptions = {}) {
    const opts: qs.IParseOptions = {
      ignoreQueryPrefix: true,
      ...options,
    };

    this._parseOptions = opts;
    this.hash = this.getHashFromParam(paramHash);
    this.parseHashStr();
  }

  /**
   * 添加参数
   */
  addParams(params: Record<string, string | number>, options: AddParamOptions = {}) {
    const opts: AddParamOptions = {
      replace: true,
      ...options,
    };

    for (const key of Object.keys(params)) {
      const isExist = this.params?.has(key);
      const value = `${params?.[key] || ''}`;

      // 数据存在且参数指定了 replace 就使用替换
      if (isExist && opts?.replace) {
        this.params?.set(key, value);
      } else {
        this.params?.append(key, value);
      }
    }

    // 更新 search
    this.updateSearch();
    this.updateHash();
    return this;
  }

  /**
   * 获取参数对象
   */
  getParams(): qs.ParsedQs {
    const params: qs.ParsedQs = {};

    for (const key of this.params.keys()) {
      params[key] = this.params.get(key)!;
    }

    return params;
  }

  /**
   * 更新hash路径
   */
  setPathName(pathStr: string) {
    // 去除hash前缀
    this.pathName = pathStr?.replace('#', '');
    this.updateHash();
    return this;
  }

  /**
   * 更新hash参数字符串
   */
  updateSearch() {
    const paramsStr = this.params?.toString();

    if (paramsStr) {
      this.search = `?${paramsStr}`;
    } else {
      this.search = '';
    }

    return this;
  }

  /**
   * 更新hash字符串
   */
  updateHash() {
    this.hash = `#${this.pathName}${this.search}`;
    return this;
  }

  /**
   * 获取正确的hash参数
   */
  getHashFromParam(paramHash: string | Location) {
    if (typeof paramHash === 'string') {
      // 存在hash前缀
      if (paramHash?.startsWith('#/')) {
        return paramHash;
      }

      return `#${paramHash}`;
    }
    // location 获取
    return paramHash?.hash || '';
  }

  /**
   * 解析当前 hash路径
   */
  parseHashStr() {
    const hashArr = this.hash?.split('?');

    if (hashArr?.length) {
      // 解析hash路径
      this.pathName = hashArr?.[0]?.replace('#', '');
    }

    // 存在hash参数
    if (hashArr?.length > 1) {
      const paramsStr = hashArr?.[1] || '';
      const paramsObj = qs.parse(paramsStr, this._parseOptions) as unknown as Record<string, string>;
      this.params = new URLSearchParams(paramsObj);
      this.updateSearch();
    }
  }

  /**
   * 获取hash字符串
   */
  toString() {
    return this.hash;
  }
}
