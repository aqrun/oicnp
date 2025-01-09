const mocks = [
  {
    pattern: '/api/test',
    method: 'GET',
    handle: (req, res) => {
      res.end('Hello world!' + req.url)
    }
  },
  {
    pattern: '/api/auth/login',
    handle: (req, res) => {
      const data = {
        username: 'alex',
        token: 'testtoken',
        uuid: 'id-abcd',
      }
      const resData = {
        code: "200",
        data: {
            data,
            total: 0,
            page: 1,
            page_size: 10,
        },
        message: ""
      }

      res.setHeader('Content-Type', 'application/json')
      res.end(JSON.stringify(resData))
    },
  },
  {
    pattern: '/api/user/list',
    handle: (req, res) => {
      const data = [
        {
          id: '1',
          username: 'alex',
          phone: '123',
        },
        {
          id: '2',
          username: 'alex',
          phone: '123',
        },
        {
          id: '3',
          username: 'alex',
          phone: '123',
        },
      ];

      const resData = {
        code: "200",
        data: {
            data,
            total: 0,
            page: 1,
            page_size: 10,
        },
        message: ''
      }

      res.setHeader('Content-Type', 'application/json')
      res.end(JSON.stringify(resData))
    },
  },
];

export default mocks;