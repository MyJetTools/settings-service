var Dialog = /** @class */ (function () {
    function Dialog() {
    }
    Dialog.show = function (data) {
        AppContext.dialogPadElement.classList.remove('dialog-pad-hidden');
        AppContext.menuElement.classList.add('blur-content');
        AppContext.contentElement.classList.add('blur-content');
        AppContext.dialogPadElement.innerHTML = this.generateDialog(data);
    };
    Dialog.populateData = function (model) {
        var el = document.getElementById('modal-content');
        el.innerHTML = this.dialogData.getContent();
        this.dialogData.populate(model);
    };
    Dialog.hide = function () {
        AppContext.dialogPadElement.classList.add('dialog-pad-hidden');
        AppContext.menuElement.classList.remove('blur-content');
        AppContext.contentElement.classList.remove('blur-content');
    };
    Dialog.onOkPressed = function () {
        var result = this.dialogData.check();
        if (result) {
            this.hide();
            this.dialogData.ok(result);
        }
    };
    Dialog.generateDialog = function (data) {
        this.dialogData = data;
        var width = data.getWidth();
        if (width == undefined) {
            width = "";
        }
        else {
            width = "style=\"width: ".concat(width, "\"");
        }
        return "\n        <div class=\"modal-dialog\" " + width + ">\n          <div class=\"modal-content\">\n            <div class=\"modal-header\">\n              <h5 class=\"modal-title\">" + data.title + "</h5>\n              <button type=\"button\" class=\"btn btn-default btn-sm\" data-bs-dismiss=\"modal\" aria-label=\"Close\" onclick=\"Dialog.hide()\"><svg class=\"bi\" width=\"1em\" height=\"1em\" fill=\"currentColor\">\n              <use xlink:href=\"bootstrap-icons.svg#x\"></use>\n              </svg></button>\n            </div>\n            <div id=\"modal-content\" class=\"modal-body\">\n                <div style=\"text-align:center;\"><img src=\"/img/loading.gif\" style=\"width: 90px;\" /></div>\n            </div>\n            <div class=\"modal-footer\">\n            <div class=\"btn-group\">\n            <button type=\"button\" class=\"btn btn-primary  btn-sm\" onclick=\"Dialog.onOkPressed()\"><svg class=\"bi\" width=\"1em\" height=\"1em\" fill=\"currentColor\">\n            <use xlink:href=\"bootstrap-icons.svg#check\"></use>\n            </svg>" + data.getOkBtnName() + "</button>\n              <button type=\"button\" class=\"btn btn-secondary btn-sm\" data-bs-dismiss=\"modal\" onclick=\"Dialog.hide()\"><svg class=\"bi\" width=\"1em\" height=\"1em\" fill=\"currentColor\">\n              <use xlink:href=\"bootstrap-icons.svg#x\"></use>\n              </svg>Cancel</button>\n            </div>\n            </div>\n          </div>\n        </div>";
    };
    return Dialog;
}());
//# sourceMappingURL=Dialog.js.map