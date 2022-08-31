var ShowYamlDialog = /** @class */ (function () {
    function ShowYamlDialog(title) {
        this.hideCancelBtn = true;
        this.title = title;
    }
    ShowYamlDialog.prototype.getWidth = function () {
        return "90%";
    };
    ShowYamlDialog.prototype.getOkBtnName = function () {
        return "Ok";
    };
    ShowYamlDialog.prototype.getContent = function () {
        return "<div class=\"form-floating\">\n        <textarea class=\"form-control\" style=\"min-height:500px;font-family: monospace;\" id=\"showYaml\" readonly=\"readonly\"></textarea>\n        <label for=\"edtYaml\">Yaml</label>\n        </div>";
    };
    ShowYamlDialog.prototype.populate = function (data) {
        document.getElementById('showYaml').innerHTML = data;
    };
    ShowYamlDialog.prototype.check = function () {
        return true;
    };
    ShowYamlDialog.prototype.ok = function (_) {
    };
    return ShowYamlDialog;
}());
//# sourceMappingURL=ShowYamlDialog.js.map