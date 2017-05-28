const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

function newPage(name){
  return new HtmlWebpackPlugin({
    filename: name + '.html',
    template: './app/views/' + name + '.html',
    inject: true,
    chunks: [name]
  })
}

module.exports = {
  // Entries have to resolve to files! they rely on Node
  // convention by default so if a directory contains *index.js*,
  // it resolves to that
  entry: {
    index: "./app/scripts/index.js",
    BinarySearchTree: "./app/scripts/BinarySearchTree.js",
    DoublyLinkList: "./app/scripts/DoublyLinkedList.js",
    HashTable: "./app/scripts/HashTable.js",
    Queue: "./app/scripts/Queue.js",
    QuickSort: "./app/scripts/QuickSort.js",
    SelectionSort: "./app/scripts/SelectionSort.js",
    Stack: "./app/scripts/Stack.js",
    BreadthFirst: "./app/scripts/BreadthFirst.js",
    DepthFirstSearch: "./app/scripts/DepthFirstSearch.js",
    SinglyLinkedList: "./app/scripts/SinglyLinkedList.js",

  },
  output: {
    path: path.resolve(__dirname, 'build/scripts'),
    filename: '[name].js',
  },
  plugins: [
    newPage('index'),
    newPage('BinarySearchTree'),
    newPage('DoublyLinkList'),
    newPage('HashTable'),
    newPage('Queue'),
    newPage('QuickSort'),
    newPage('SelectionSort'),
    newPage('Stack'),
    newPage('BreadthFirst'),
    newPage('SinglyLinkedList'),
    newPage('DepthFirstSearch'),
  ],

  rules: [
    {
      test: /\.js$/,
      exclude: /(node_modules)/,
      use: {
        loader: 'babel-loader',
        options: {
          presets: ['env'],
          plugins: [require('babel-plugin-transform-object-rest-spread')]
        }
      }
    }
  ]
};