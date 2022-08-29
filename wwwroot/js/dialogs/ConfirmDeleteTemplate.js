var ConfirmDeleteTemplate = /** @class */ (function () {
    function ConfirmDeleteTemplate(title, data) {
        this.title = title;
        this.data = data;
    }
    ConfirmDeleteTemplate.prototype.getWidth = function () {
        return "600px";
    };
    ConfirmDeleteTemplate.prototype.getOkBtnName = function () {
        return "Confirm";
    };
    ConfirmDeleteTemplate.prototype.getContent = function () {
        return "<h4>You are about to delete template</h4>";
    };
    ConfirmDeleteTemplate.prototype.populate = function () {
    };
    ConfirmDeleteTemplate.prototype.check = function () {
        return true;
    };
    ConfirmDeleteTemplate.prototype.ok = function (_) {
        $.ajax({ type: "POST", url: "/api/templates/delete", data: this.data })
            .then(function () {
            Actions.loadTemplates();
        })
            .fail(function () {
        });
    };
    return ConfirmDeleteTemplate;
}());
//# sourceMappingURL=ConfirmDeleteTemplate.js.map