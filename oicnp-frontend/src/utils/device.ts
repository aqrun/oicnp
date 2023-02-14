import { GetServerSidePropsContext } from 'next';

export const checkIsMobile = (ctx: GetServerSidePropsContext) => {
  const isServer = !!ctx.req;
  const userAgentInfo = isServer
    ? ctx?.req?.headers['user-agent']
    : navigator.userAgent
  
  if (!userAgentInfo) return false;
 
  const mobileAgents = ['Android', 'iPhone',
    'SymbianOS', 'Windows Phone',
    'iPad', 'iPod',
  ];

  let mobile_flag = false;

  for (let i = 0; i < mobileAgents?.length; i++) {
    if (userAgentInfo?.indexOf(mobileAgents[i]) > 0) {
      mobile_flag = true;
      break;
    }
  }

  return mobile_flag;
};