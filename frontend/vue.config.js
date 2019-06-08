const mockedWebpackFunction = require('./mockConfig')

module.exports = {
    configureWebpack: {
        devServer: {
            before: mockedWebpackFunction
        },
    }
  }