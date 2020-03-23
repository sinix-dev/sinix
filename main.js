const express = require('express');
const graphqlHTTP = require('express-graphql');
var { buildSchema } = require('graphql');

const app = express();

var schema = buildSchema(`
  type Query {
    hello: String
  }
`);

var root = {
  hello: () => {
    return 'Hello world!';
  },
};

app.use(
  '/graphql',
  graphqlHTTP({
    schema: schema,
    graphiql: true,
    rootValue: root
  }),
);

app.listen(4000);
