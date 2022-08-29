var Dialog = /** @class */ (function () {
    function Dialog() {
    }
    Dialog.show = function (data) {
        AppContext.dialogPadElement.classList.remove('dialog-pad-hidden');
        AppContext.menuElement.classList.add('blur-content');
        AppContext.contentElement.classList.add('blur-content');
        AppContext.dialogPadElement.innerHTML = this.generateDialog(data);
        data.populate();
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
        return "\n        <div class=\"modal-dialog\" " + width + ">\n          <div class=\"modal-content\">\n            <div class=\"modal-header\">\n              <h5 class=\"modal-title\">" + data.title + "</h5>\n              <button type=\"button\" class=\"btn btn-default\" data-bs-dismiss=\"modal\" aria-label=\"Close\" onclick=\"Dialog.hide()\">X</button>\n            </div>\n            <div class=\"modal-body\">\n                " + data.getContent() + "\n            </div>\n            <div class=\"modal-footer\">\n            <button type=\"button\" class=\"btn btn-primary\" onclick=\"Dialog.onOkPressed()\">Save changes</button>\n              <button type=\"button\" class=\"btn btn-secondary\" data-bs-dismiss=\"modal\" onclick=\"Dialog.hide()\">Cancel</button>\n            </div>\n          </div>\n        </div>";
    };
    return Dialog;
}());
//# sourceMappingURL=Dialog.js.map