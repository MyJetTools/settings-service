var ShowUsageSecretOfSecrets = /** @class */ (function () {
    function ShowUsageSecretOfSecrets(title, secretName) {
        this.hideCancelBtn = true;
        this.title = title;
        this.secretName = secretName;
    }
    ShowUsageSecretOfSecrets.prototype.getWidth = function () {
        return "90%";
    };
    ShowUsageSecretOfSecrets.prototype.getOkBtnName = function () {
        return "Ok";
    };
    ShowUsageSecretOfSecrets.prototype.getContent = function () {
        return "\n        <div class=\"form-control\" style=\"height:500px;font-family: monospace; overflow-y: scroll;\" id=\"secretUsage\" readonly=\"readonly\"></div>\n        ";
    };
    ShowUsageSecretOfSecrets.prototype.populate = function (data) {
        var result = "";
        for (var _i = 0, data_1 = data; _i < data_1.length; _i++) {
            var itm = data_1[_i];
            result += '<div>' + itm.name + ': ' + itm.value + '</div>';
        }
        document.getElementById('secretUsage').innerHTML = result;
    };
    ShowUsageSecretOfSecrets.prototype.check = function () {
        return true;
    };
    ShowUsageSecretOfSecrets.prototype.ok = function (_) {
    };
    return ShowUsageSecretOfSecrets;
}());
//# sourceMappingURL=ShowUsageSecretOfSecrets.js.map