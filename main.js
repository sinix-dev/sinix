const path = require('path')
const express = require('express');
const graphqlHTTP = require('express-graphql');

const schema = require('./src/schema')
const root = require('./src/resolver')

const app = express();

app.use(
  '/graphql',
  graphqlHTTP({
    schema: schema,
    graphiql: true,
    rootValue: root
  }),
);

app.use(express.static(path.join(__dirname, 'public')))

app.listen(4000);