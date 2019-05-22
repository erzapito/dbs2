const seriesResponse = require('./mocks/seriesResponse');

module.exports = function(app, server) {
    app.get('/some/path', function(req, res) {
        res.json({ custom: 'response' });
    });
    app.get('/api/series', function(req, res){
        res.json(seriesResponse);
    });
};
