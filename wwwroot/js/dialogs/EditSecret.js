var EditSecretDialog = /** @class */ (function () {
    function EditSecretDialog(title, viewModel) {
        this.title = title;
        this.viewModel = viewModel;
    }
    EditSecretDialog.prototype.getWidth = function () {
        return "600px";
    };
    EditSecretDialog.prototype.getOkBtnName = function () {
        return "Save";
    };
    EditSecretDialog.prototype.getContent = function () {
        return "\n        <div class=\"form-floating mb-3\">\n        <input class=\"form-control\" id=\"edtName\" name=\"name\">\n        <label for=\"edtName\">Name</label>\n        </div>\n\n      <div class=\"form-floating\">\n      <input class=\"form-control\" id=\"edtSecret\" name=\"secret\">\n      <label for=\"edtSecret\">Secret</label>\n      </div>";
    };
    EditSecretDialog.prototype.populate = function () {
        this.edtName = document.getElementById('edtName');
        this.edtSecret = document.getElementById('edtSecret');
        if (this.viewModel) {
            this.edtName.value = this.viewModel.name;
            this.edtName.readOnly = true;
            this.edtSecret.value = this.viewModel.secret;
        }
    };
    ;
    EditSecretDialog.prototype.check = function () {
        if (!passElement(this.edtName)) {
            return undefined;
        }
        if (!passElement(this.edtSecret)) {
            return undefined;
        }
        return {
            name: this.edtName.value,
            secret: this.edtSecret.value
        };
    };
    EditSecretDialog.prototype.ok = function (data) {
        $.ajax({ type: "POST", url: "/api/secrets/post", data: data })
            .then(function () {
            Actions.loadSecrets();
        })
            .fail(function () {
        });
    };
    return EditSecretDialog;
}());
//# sourceMappingURL=EditSecret.js.map