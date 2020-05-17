const path = require('path');

const webpack = require('webpack');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = (env, args) => {
  const isProd = args.mode === 'production';

  return {
    entry: './www/index.js',
    output: {
      path: path.resolve(__dirname, 'dist'),
      filename: 'fluidweb.js',
    },
    module: {
      rules: [
        {
          test: /\.(sf2|png)$/i,
          use: [
            {
              loader: 'file-loader'
            }
          ]
        }
      ],
    },
    plugins: [
      new HtmlWebpackPlugin({
        template: './www/index.html',
      }),
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, '.'),
        outName: "fluidweb",
        outDir: path.resolve(__dirname, "./fluidweb"),
        forceMode: isProd ? "production" : "development",
      }),
      new webpack.ProvidePlugin({
        TextDecoder: ['text-decoding', 'TextDecoder'],
        TextEncoder: ['text-encoding', 'TextEncoder'],
      })
    ]
  };
};

