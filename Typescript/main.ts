
class main {
    private static body: HTMLElement;


    private static windowHeight: number;
    private static windowWidth: number;

    static init() {
        this.body = document.getElementsByTagName('body')[0];
        this.body.innerHTML = HtmlMain.generateLayout();
        AppContext.menuElement = document.getElementById('menu-bar');
        AppContext.contentElement = document.getElementById('content');
        AppContext.dialogPadElement = document.getElementById('dialog-pad');
    }




    static resize() {

        let height = window.innerHeight;
        let width = window.innerWidth;


        if (this.windowHeight == height && this.windowWidth == width)
            return;

        this.windowHeight = height;
        this.windowWidth = width;



        let position = 'top:0; left:0; width:' + width + 'px; height:' + height + 'px';
        AppContext.contentElement.setAttribute('style',
            position);

        AppContext.dialogPadElement.setAttribute('style',
            position);


        position = 'top:0; left:0; width: 200px; height:' + height + 'px';
        AppContext.menuElement.setAttribute('style',
            position);

    }

    static background() {
        this.resize();
    }
}

window.setTimeout(() => {
    main.init();
    main.background();
}, 100);


window.setInterval(() => main.background(), 1000);
