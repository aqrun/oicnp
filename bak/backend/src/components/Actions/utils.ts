import React, {
  Children,
  isValidElement,
  ReactNode,
  ReactElement,
} from 'react'

/**
 * `flattern the React.Fragment inside children:
 * children: [<A>, <B>, <React.Fragment><C><D></React.Fragment>, <E>]
 * =>
 * result: [<A>, <B>, <C>, <D>, <E>]
 *
 * recursive option explain:
 * children: [<A>,
 *  <B>,
 *  <React.Fragment>
 *    <C>
 *    <React.Fragment><D></React.Fragment>
 *  </React.Fragment>,
 * <E>]
 * =>
 * if recursive: true, result will be:  [<A>, <B>, <C>, <D>, <E>] ()
 * if recursive: false, result will be: [<A>, <B>, <C>, <React.Fragment><D></React.Fragment>, <E>]`
 */

/**
 * @public
 */
export function spreadFragmentInChildren(
  children: ReactNode,
  recursive = true
) {
  if (!isValidElement(children) && !Array.isArray(children)) {
    throw new Error(`expect a single react element of an array of element`)
  }
  const result: ReactNode[] = []
  // children can be a single element or array of it
  Children.forEach(children, (node) => {
    if (!isValidElement(node)) {
      // node maybe 0, '', boolean, null, undefined
      // React.Children.forEach auto spread array into nodes, so node can't be array
      result.push(node)
    } else if (
      node.type === React.Fragment &&
      node.props &&
      (node.props as any).children
    ) {
      // node.props.children can be a single element or array of it
      if (recursive) {
        result.push(...spreadFragmentInChildren((node.props as any).children))
      } else if (Array.isArray((node.props as any).children)) {
        result.push(...(node.props as any).children)
      } else result.push((node.props as any).children)
    } else {
      result.push(node)
    }
  })
  return result
}
