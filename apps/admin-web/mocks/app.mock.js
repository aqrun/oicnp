const mocks = [
  {
    pattern: '/api/test',
    method: 'GET',
    handle: (req, res) => {
      res.end('Hello world!' + req.url)
    }
  },
  {
    pattern: '/api/users',
    handle: (req, res) => {
      const data = {
        name: 'alex',
      }
      res.setHeader('Content-Type', 'application/json')
      res.end(JSON.stringify(data))
    },
  },
  {
    pattern: '/api/auth/login',
    handle: (req, res) => {
      const data = {
        username: 'alex',
        token: 'testtoken',
        uuid: 'id-abcd',
      }
      res.setHeader('Content-Type', 'application/json')
      res.end(JSON.stringify(data))
    },
  },
];

export default mocks;