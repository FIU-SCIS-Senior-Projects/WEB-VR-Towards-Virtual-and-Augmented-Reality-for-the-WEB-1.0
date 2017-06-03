/* gulpfile.js file */

const gulp = require('gulp')
const gutil = require('gulp-util')
const rename = require('gulp-rename')

const jshint = require('gulp-jshint')
const eslint = require('gulp-eslint')

const browserify = require('browserify')
const watchify = require('watchify')
const babelify = require('babelify')
const babel = require('gulp-babel')
const livereload = require('gulp-livereload')
const merge = require('merge')
const source = require('vinyl-source-stream')
const buffer = require('vinyl-buffer')
const sourceMaps = require('gulp-sourcemaps')
const es = require('event-stream')

const glob = require('glob')

const config = {
  js: {
    src: './app/scripts/index.js',
    outputDir: './app/buildScripts/',
  }
}

const paths = {
  scripts: ['./app/scripts/**/*.js', '!node_modules/**/*.js', '!./app/buildScripts/**/*.js', '!webpack.config.js']
}

  var files = [
    './app/scripts/index.js',
    './app/scripts/BinarySearchTree.js',
    './app/scripts/BreadthFirst.js',
    './app/scripts/DepthFirstSearch.js',
    './app/scripts/DoublyLinkedList.js',
    './app/scripts/HashTable.js',
    './app/scripts/Queue.js',
    './app/scripts/QuickSort.js',
    './app/scripts/SelectionSort.js',
    './app/scripts/SinglyLinkedList.js',
    './app/scripts/Stack.js'
  ]

gulp.task('default', function() {
  return gutil.log('Gulp is running')
})

gulp.task('bundle', function() {
  // define our files which we want to be bundled


  // map them to our stream function
  var tasks = files.map(function(entry) {
    return browserify({ 
      entries: [entry],
      fullPaths: false 
      })
      .bundle()
      .pipe(source(entry))
      //rename them to have bundle as postfix
      .pipe(rename({
        extname: '.bundle.js'
      }))
      .pipe(gulp.dest('./app/build'))
  })

  // Create a merged stream
  return es.merge.apply(null, tasks)
})

gulp.task('lint-client', () =>
     gulp.src(paths.scripts)
    .pipe(eslint())
    .pipe(eslint.format())
    .pipe(eslint.failAfterError()))

gulp.task('lint-test', () => gulp.src('./test/**/*.js')
    .pipe(jshint())
    .pipe(jshint.reporter('default')))

gulp.task('watch', () => {
  gulp.watch('/app/scripts/*.js', ['lint-client', 'es5', 'bundle'])
})


gulp.task('es5', function() {
  return gulp.src((paths.scripts)
    .pipe(babel({
      presets: ['es2015']
    }))
    .pipe(gulp.dest('buildScripts'))
  ) }
)

gulp.task('build:pages', done => {
  glob('./app/scripts/**/*.js', (err, files) => {
    if (err) {
      done(err);
    }
    const tasks = files.map(entry => {
      const b = browserify({
        entries: [entry],
        extensions: ['.js'],
        debug: true,
        cache: {},
        packageCache: {},
        fullPaths: true
      })
      .plugin(watchify);

      const bundle = () => {
        return b.bundle()
          .pipe(source(entry))
          .pipe(rename({
            extname: '.bundle.js'
            }))
          .pipe(buffer())
          .pipe(gulp.dest('./app/build/'));
      };

      b.on('update', bundle);
      return bundle();
    });
    es.merge(tasks).on('end', done);
  });
});



