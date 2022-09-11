var ShowSecretUsageDialog = /** @class */ (function () {
    function ShowSecretUsageDialog(title, secretName) {
        this.hideCancelBtn = true;
        this.title = title;
        this.secretName = secretName;
    }
    ShowSecretUsageDialog.prototype.getWidth = function () {
        return "90%";
    };
    ShowSecretUsageDialog.prototype.getOkBtnName = function () {
        return "Ok";
    };
    ShowSecretUsageDialog.prototype.getContent = function () {
        return "\n        <div class=\"form-control\" style=\"height:500px;font-family: monospace; overflow-y: scroll;\" id=\"secretUsage\" readonly=\"readonly\"></div>\n        ";
    };
    ShowSecretUsageDialog.prototype.populate = function (data) {
        var result = "";
        for (var _i = 0, data_1 = data; _i < data_1.length; _i++) {
            var itm = data_1[_i];
            result += '<h4>' + itm.env + '/' + itm.name + '</h4>';
            for (var _a = 0, _b = itm.yaml.split(/\r?\n/); _a < _b.length; _a++) {
                var line = _b[_a];
                if (line.indexOf(this.secretName) >= 0) {
                    result += '<div>';
                    for (var i = 0; i < spacesAmount(line); i++) {
                        result += '&nbsp;';
                    }
                    result += '<b>' + line.trim() + '</b></div>';
                }
                else {
                    result += '<div style="color:gray">';
                    for (var i = 0; i < spacesAmount(line); i++) {
                        result += '&nbsp;';
                    }
                    result += line.trim() + '</div>';
                }
            }
            result += '<hr/>';
        }
        document.getElementById('secretUsage').innerHTML = result;
    };
    ShowSecretUsageDialog.prototype.check = function () {
        return true;
    };
    ShowSecretUsageDialog.prototype.ok = function (_) {
    };
    return ShowSecretUsageDialog;
}());
function splitByLines(text) {
    return text.split(/\r?\n/);
}
function spacesAmount(text) {
    var result = 0;
    for (var i = 0; i < text.length; i++) {
        if (text[i] == ' ') {
            result++;
        }
        else {
            break;
        }
    }
    return result;
}
//# sourceMappingURL=ShowSecretUsage.js.map