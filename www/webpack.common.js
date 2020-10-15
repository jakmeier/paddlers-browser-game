const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
    entry: "./bootstrap.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "bootstrap.js",
    },
    resolve: {
        extensions: ['.mjs', '.js', '.svelte'],
        mainFields: ['svelte', 'browser', 'module', 'main'],
        modules: [path.resolve(__dirname, 'node_modules'), 'node_modules']
    },
    module: {
        rules: [{
            test: /\.(html|svelte)$/,
            exclude: /node_modules/,
            use: 'svelte-loader'
        }, {
            test: /\.css$/,
            use: [
                'style-loader',
                'css-loader'
            ]
        }]
    },
    plugins: [
        new CopyWebpackPlugin(['index.html'])
    ],
};