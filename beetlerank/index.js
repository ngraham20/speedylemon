var express = require('express');
const app = express();
const hostname = 'localhost';
const port = 3000;

const devapi = require('./api')

app.use('/api/dev', devapi);

app.listen(port, hostname);
