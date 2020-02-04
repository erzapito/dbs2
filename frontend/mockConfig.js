const seriesResponse0 = require('./mocks/seriesResponse0');
const seriesResponse1 = require('./mocks/seriesResponse1');
const seriesResponse2 = require('./mocks/seriesResponse2');

const musicResponse0 = require('./mocks/musicResponse0');
const musicResponse1 = require('./mocks/musicResponse1');
const musicResponse2 = require('./mocks/musicResponse2');

module.exports = function(app) {
    // series
    app.get('/api/series', function(req, res){
        if (!req.query.page) {
            res.json(seriesResponse0);
        } else if (req.query.page == 0) {
            res.json(seriesResponse0);
        } else if (req.query.page == 1) {
            res.json(seriesResponse1);
        } else {
            res.json(seriesResponse2)
        }
    });
    app.post('/api/series', function(req,res){
        res.json();
    });
    app.delete('/api/series/*', function(req,res){
        res.json();
    });
    app.put('/api/series/*', function(req,res){
        res.json();
    });

    // music
    app.get('/api/music', function(req, res){
        if (!req.query.page) {
            res.json(musicResponse0);
        } else if (req.query.page == 0) {
            res.json(musicResponse0);
        } else if (req.query.page == 1) {
            res.json(musicResponse1);
        } else {
            res.json(musicResponse2)
        }
    });
    app.post('/api/music', function(req,res){
        res.json();
    });
    app.delete('/api/music/*', function(req,res){
        res.json();
    });
    app.put('/api/music/*', function(req,res){
        res.json();
    });
};
