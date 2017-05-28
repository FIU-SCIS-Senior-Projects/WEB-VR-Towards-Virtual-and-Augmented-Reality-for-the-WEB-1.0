const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

const scriptsPath = "./app/scripts/"

function newPage(name){
  return new HtmlWebpackPlugin({
    filename: name + '.html',
    template: './app/views/' + name + '.html',
    inject: true,
    chunks: [name]
  })
}

const PATHS = {
  index: "./app/scripts/index.js",
  bst: "./app/scripts/BinarySearchTree.js",
  breadthFirst: scriptsPath + "BreadthFirst.js",
  depthFirst: scriptsPath + "DepthFirst.js",
  dblyLinkedList: scriptsPath + "DoublyLinkedList.js",
  hashTable: scriptsPath + "HashTable.js",
  singlyLinkList: scriptsPath + "LinkedList.js",
  queue: scriptsPath + "Queue.js",
  quickSort: scriptsPath + "QuickSort.ja",
  selectionSort: scriptsPath + "SelectionSort.js",
  stack: scriptsPath + "Stack.js",

  build: path.resolve(__dirname, 'build'),
  source: "./app/views/"
};

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
    path: path.resolve(__dirname, 'build'),
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
};