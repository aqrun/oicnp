import {
  DescribeUserListResponseData,
} from '../src/api';

const mocks = [
  {
    pattern: '/api/user/list',
    handle: (req, res) => {
      const params = req?.body;
      const data: DescribeUserListResponseData['data'] = [];

      for (let i = 0; i < 10; i++) {
        data.push({
          id: `${i}`,
          username: `username ${i}-${params?.page}`,
          phone: '18049475554',
        });
      }

      const resData = {
        code: "200",
        data: {
            data,
            total: 30,
            page: params?.page || 1,
            page_size: params?.page_size || 10,
        },
        message: ''
      }

      res.setHeader('Content-Type', 'application/json')
      res.end(JSON.stringify(resData))
    },
  },
  {
    pattern: '/api/user/remove',
    handle: (req, res) => {
      const resData = {
        code: "200",
        data: {
            uid: '123'
        },
        message: ''
      }

      res.setHeader('Content-Type', 'application/json')
      res.end(JSON.stringify(resData))
    },
  },
];

export default mocks;