import path from "path";
import sveltePreprocess from "svelte-preprocess";
import webpack from "webpack";
import HtmlWebpackPlugin from "html-webpack-plugin";

interface Argv {
  mode?: "production" | "development";
}

const tsRule = {
  test: /\.ts$/,
  exclude: /node_modules/,
  use: {
    loader: "ts-loader",
    options: {
      compilerOptions: {
        noEmit: false,
        module: "es6",
      },
    },
  },
};

function electronMain(_env: unknown, argv: Argv): webpack.Configuration {
  const mode = argv.mode || "development";
  return {
    name: "main",
    entry: "./native/index.ts",
    mode,
    cache: {
      type: "filesystem",
    },
    externals: ["fsevents"],
    target: "electron-main",
    externalsPresets: { electronMain: true, node: true },
    module: {
      rules: [tsRule],
    },
    resolve: {
      extensions: [".ts", ".js"],
    },
    output: {
      filename: "bundle.js",
      path: path.resolve(__dirname, "native"),
    },
    optimization: {
      minimize: false,
    },
  };
}

function ui(_env: unknown, argv: Argv): webpack.Configuration {
  const mode = argv.mode || "development";
  const isProduction = mode === "production";
  return {
    name: "ui",
    entry: "./ui/index.ts",
    mode,
    devtool: isProduction ? "source-map" : "eval-source-map",
    cache: {
      type: "filesystem",
    },
    performance: {
      hints: false,
    },
    target: "web",
    module: {
      rules: [
        {
          test: /\.svelte$/,
          use: {
            loader: "svelte-loader",
            options: {
              compilerOptions: { dev: !isProduction },
              preprocess: sveltePreprocess(),
            },
          },
        },
        tsRule,
      ],
    },
    resolve: {
      fallback: {
        crypto: require.resolve("crypto-browserify"),
        stream: require.resolve("stream-browserify"),
      },
      extensions: [".svelte", ".ts", ".js"],
    },
    output: {
      filename: "bundle.js",
      path: path.resolve(__dirname, "public"),
    },
    plugins: [
      new webpack.ProvidePlugin({
        Buffer: ["buffer", "Buffer"],
        process: "process",
      }),
      new HtmlWebpackPlugin({
        template: "ui/index.html",
        meta: {
          "Content-Security-Policy": {
            "http-equiv": "Content-Security-Policy",
            content: isProduction
              ? "script-src 'self'"
              : "script-src 'self' 'unsafe-eval'",
          },
        },
      }),
    ],
  };
}

export default [ui, electronMain];
