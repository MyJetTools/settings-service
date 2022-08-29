
var gulp = require('gulp');
var minifyjs = require('gulp-js-minify');
var concat = require('gulp-concat');

gulp.task('default', function () {
    return gulp
        .src([
            './wwwroot/js/HtmlStaticElement.js',
            './wwwroot/js/AppContext.js',
            './wwwroot/js/Dialog.js',
            './wwwroot/js/dialogs/EditTemplate.js',
            './wwwroot/js/dialogs/EditSecret.js',
            './wwwroot/js/dialogs/ConfirmDeleteTemplate.js',
            './wwwroot/js/Actions.js',
            './wwwroot/js/HtmlMain.js',
            './wwwroot/js/main.js'])
        .pipe(minifyjs())
        .pipe(concat('app.js'))
        .pipe(gulp.dest('./wwwroot/js/'))
});