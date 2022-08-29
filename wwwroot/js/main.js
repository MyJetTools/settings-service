var main = /** @class */ (function () {
    function main() {
    }
    main.init = function () {
        this.body = document.getElementsByTagName('body')[0];
        this.body.innerHTML = HtmlMain.generateLayout();
        AppContext.menuElement = document.getElementById('menu-bar');
        AppContext.contentElement = document.getElementById('content');
        AppContext.dialogPadElement = document.getElementById('dialog-pad');
    };
    main.resize = function () {
        var height = window.innerHeight;
        var width = window.innerWidth;
        if (this.windowHeight == height && this.windowWidth == width)
            return;
        this.windowHeight = height;
        this.windowWidth = width;
        var position = 'top:0; left:0; width:' + width + 'px; height:' + height + 'px';
        AppContext.contentElement.setAttribute('style', position);
        AppContext.dialogPadElement.setAttribute('style', position);
        position = 'top:0; left:0; width: 200px; height:' + height + 'px';
        AppContext.menuElement.setAttribute('style', position);
    };
    main.background = function () {
        this.resize();
    };
    return main;
}());
window.setTimeout(function () {
    main.init();
    main.background();
}, 100);
window.setInterval(function () { return main.background(); }, 1000);
//# sourceMappingURL=main.js.map