const seriesResponse0 = require('./mocks/seriesResponse0');
const seriesResponse1 = require('./mocks/seriesResponse1');
const seriesResponse2 = require('./mocks/seriesResponse2');

module.exports = function(app, server) {
    app.get('/api/series', function(req, res){
        if (!req.query.page) {
            res.json(seriesResponse0);
        } else if (req.query.page == 0) {
            console.log(seriesResponse0);
            res.json(seriesResponse0);
        } else if (req.query.page == 1) {
            res.json(seriesResponse1);
        } else {
            res.json(seriesResponse2)
        }
    });
};
