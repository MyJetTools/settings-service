var EditSecretDialog = /** @class */ (function () {
    function EditSecretDialog(title) {
        this.hideCancelBtn = false;
        this.title = title;
    }
    EditSecretDialog.prototype.getWidth = function () {
        return "600px";
    };
    EditSecretDialog.prototype.getOkBtnName = function () {
        return "Save";
    };
    EditSecretDialog.prototype.getContent = function () {
        return "\n        <div class=\"form-floating mb-3\">\n        <input class=\"form-control\" id=\"edtName\" name=\"name\">\n        <label for=\"edtName\">Name</label>\n        </div>\n\n      <div class=\"form-floating mb-3\">\n      <input class=\"form-control\" id=\"edtSecret\" name=\"secret\">\n      <label for=\"edtSecret\">Secret</label>\n      </div>\n\n      <div class=\"form-floating mb-3\">\n      <input class=\"form-control\" id=\"edtRemoteValue\" name=\"remote_value\">\n      <label for=\"edtRemoteValue\">Remote value (optional)</label>\n      </div>\n\n\n      <div class=\"form-floating mb-3\">\n      <input class=\"form-control\" id=\"edtLevel\" name=\"secret\">\n      <label for=\"edtLevel\">Level</label>\n      </div>";
    };
    EditSecretDialog.prototype.populate = function (viewModel) {
        this.viewModel = viewModel;
        this.edtName = document.getElementById('edtName');
        this.edtSecret = document.getElementById('edtSecret');
        this.edtRemoteValue = document.getElementById('edtRemoteValue');
        this.edtLevel = document.getElementById('edtLevel');
        if (this.viewModel) {
            this.edtName.value = this.viewModel.name;
            this.edtName.readOnly = true;
            this.edtSecret.value = this.viewModel.secret;
            this.edtRemoteValue.value = this.viewModel.remote_value || "";
            this.edtLevel.value = this.viewModel.level.toString();
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
        if (!passElement(this.edtLevel)) {
            return undefined;
        }
        return {
            name: this.edtName.value,
            secret: this.edtSecret.value,
            remote_value: this.edtRemoteValue.value,
            level: parseInt(this.edtLevel.value)
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