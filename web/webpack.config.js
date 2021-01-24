const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const CopyPlugin = require("copy-webpack-plugin");
const { CleanWebpackPlugin } = require('clean-webpack-plugin');

module.exports = {
    entry: '../target/wasm.js',
    target: ["web", "es2020"],
    experiments: {
        asyncWebAssembly: true,
    },
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'wasm.js',
    },
    plugins: [
        new CleanWebpackPlugin(),
        new HtmlWebpackPlugin({ template: './index.html' }),
        new CopyPlugin({
            patterns: [
                { from: "../assets", to: "assets" },
            ],
        })
    ],
    devServer: {
        contentBase: path.join(__dirname, '../assets'),
        contentBasePublicPath: '/assets'
    }
};