
var log = require('git-log');
log(__dirname + '/main.rs').on('data', console.log);

