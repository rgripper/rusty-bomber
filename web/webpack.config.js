const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyPlugin = require("copy-webpack-plugin");

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
        new HtmlWebpackPlugin({ template: './index.html' }),
        new CopyPlugin({
            patterns: [
                { from: "../assets", to: "dist" },
            ],
        }),
        // new WasmPackPlugin({
        //     crateDirectory: path.resolve(__dirname, "..")
        // })
    ],
    devServer: {
        contentBase: path.join(__dirname, '../assets'),
        contentBasePublicPath: '/assets'
    }
};