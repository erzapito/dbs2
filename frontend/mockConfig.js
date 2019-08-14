const seriesResponse = require('./mocks/seriesResponse');

module.exports = function(app, server) {
    app.get('/api/series', function(req, res){
        if (!req.query.page) {
            res.json(seriesResponse[0]);
        } else if (req.query.page == 0) {
            console.log(seriesResponse[0]);
            res.json(seriesResponse[0]);
        } else if (req.query.page == 1) {
            res.json(seriesResponse[1]);
        } else {
            res.json(seriesResponse[2])
        }
    });
};
