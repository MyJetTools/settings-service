var EditTemlateDialog = /** @class */ (function () {
    function EditTemlateDialog(title) {
        this.title = title;
    }
    EditTemlateDialog.prototype.getWidth = function () {
        return undefined;
    };
    EditTemlateDialog.prototype.getOkBtnName = function () {
        return "Save";
    };
    EditTemlateDialog.prototype.getContent = function () {
        return "\n        <div class=\"form-floating mb-3\">\n        <input class=\"form-control\" id=\"edtEnv\" name=\"env\">\n        <label for=\"edtEnv\">Env</label>\n        </div>\n\n        <div class=\"form-floating mb-3\">\n        <input class=\"form-control\" id=\"edtName\" name=\"name\">\n        <label for=\"edtName\">Name</label>\n        </div>\n\n        <div class=\"form-floating\">\n        <textarea class=\"form-control\" style=\"min-height:500px;font-family: monospace;\" id=\"edtYaml\"  name=\"yaml\"></textarea>\n        <label for=\"edtYaml\">Yaml</label>\n        </div>";
    };
    EditTemlateDialog.prototype.populate = function (viewModel) {
        this.viewModel = viewModel;
        this.edtEnv = document.getElementById('edtEnv');
        this.edtName = document.getElementById('edtName');
        this.edtYaml = document.getElementById('edtYaml');
        if (this.viewModel) {
            this.edtEnv.value = this.viewModel.env;
            this.edtEnv.readOnly = true;
            this.edtName.value = this.viewModel.name;
            this.edtName.readOnly = true;
            this.edtYaml.value = this.viewModel.yaml;
        }
    };
    ;
    EditTemlateDialog.prototype.check = function () {
        if (!passElement(this.edtEnv)) {
            return undefined;
        }
        if (!passElement(this.edtName)) {
            return undefined;
        }
        if (!passElement(this.edtYaml)) {
            return undefined;
        }
        return {
            env: this.edtEnv.value,
            name: this.edtName.value,
            yaml: this.edtYaml.value
        };
    };
    EditTemlateDialog.prototype.ok = function (data) {
        $.ajax({ type: "POST", url: "/api/templates/post", data: data })
            .then(function () {
            Actions.loadTemplates();
        })
            .fail(function () {
        });
    };
    return EditTemlateDialog;
}());
function passElement(edt) {
    console.log(edt.id);
    console.log(edt.value);
    if (edt.value == "") {
        edt.focus();
        edt.select();
        return false;
    }
    return true;
}
//# sourceMappingURL=EditTemplate.js.map