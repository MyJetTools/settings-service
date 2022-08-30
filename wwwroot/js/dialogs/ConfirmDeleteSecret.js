var ConfirmDeleteSecret = /** @class */ (function () {
    function ConfirmDeleteSecret(title, data) {
        this.title = title;
        this.data = data;
    }
    ConfirmDeleteSecret.prototype.getWidth = function () {
        return "600px";
    };
    ConfirmDeleteSecret.prototype.getOkBtnName = function () {
        return "Confirm";
    };
    ConfirmDeleteSecret.prototype.getContent = function () {
        return "<h4>You are about to delete secret</h4>";
    };
    ConfirmDeleteSecret.prototype.populate = function () {
    };
    ConfirmDeleteSecret.prototype.check = function () {
        return true;
    };
    ConfirmDeleteSecret.prototype.ok = function (_) {
        $.ajax({ type: "POST", url: "/api/secrets/delete", data: this.data })
            .then(function () {
            Actions.loadTemplates();
        })
            .fail(function () {
        });
    };
    return ConfirmDeleteSecret;
}());
//# sourceMappingURL=ConfirmDeleteSecret.js.map