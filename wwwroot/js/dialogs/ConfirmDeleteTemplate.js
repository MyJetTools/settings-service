var ConfirmDeleteTemplate = /** @class */ (function () {
    function ConfirmDeleteTemplate(title) {
        this.title = title;
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
    ConfirmDeleteTemplate.prototype.populate = function (data) {
        this.data = data;
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