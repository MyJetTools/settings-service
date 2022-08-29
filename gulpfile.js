
var gulp = require('gulp');
var minifyjs = require('gulp-js-minify');
var concat = require('gulp-concat');

gulp.task('default', function () {
    return gulp
        .src(['./wwwroot/js/Utils.js',
            './wwwroot/js/HtmlStatusBar.js',
            './wwwroot/js/HtmlStaticElement.js',
            './wwwroot/js/HtmlGraph.js',
            './wwwroot/js/HtmlServices.js',
            './wwwroot/js/HtmlTunnelTraffic.js',
            './wwwroot/js/HtmlServices.js',
            './wwwroot/js/HtmlMain.js',
            './wwwroot/js/main.js'])
        .pipe(minifyjs())
        .pipe(concat('app.js'))
        .pipe(gulp.dest('./wwwroot/js/'))
});