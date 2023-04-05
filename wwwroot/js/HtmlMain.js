var HtmlMain = /** @class */ (function () {
    function HtmlMain() {
    }
    HtmlMain.generateLayout = function () {
        return "<div id=\"content\"></div><div id=\"menu-bar\" style=\"width:200px;\">" + this.generateStatusBar() +
            "</div><div id=\"dialog-pad\" class=\"dialog-pad-hidden\"></div>";
    };
    HtmlMain.generateStatusBar = function () {
        var env = document.body.getAttribute("data-env");
        var envColor = document.body.getAttribute("data-env-color");
        return "<div class=\"title\"> <h3>Settings </h3><h4 style=\"text-shadow: 1px 1px 1px black;color: " + envColor + "\">" + env + "</h4></div> "
            + "<div class=\"menu-item\" data-itm=\"Secrets\" onclick=\"Actions.menuClicked(this)\"> Secrets </div>"
            + "<div class=\"menu-item\" data-itm=\"Templates\" onclick=\"Actions.menuClicked(this)\">Templates</div>"
            + "<div class=\"menu-item\" data-itm=\"SubTemplates\" onclick=\"Actions.menuClicked(this)\">Sub templates</div>";
    };
    HtmlMain.generateTemplateContent = function (templates) {
        var result = "<table class=\"table table-striped\"><tr><th>Env</th><th>Name</th><th>Created</th><th>Updated</th><th>LastRequest</th><th><button class=\"btn btn-sm btn-primary\" onclick=\"Actions.addTemplate()\"><svg class=\"bi\" width=\"1em\" height=\"1em\" fill=\"currentColor\">\n        <use xlink:href=\"bootstrap-icons.svg#plus-circle-dotted\"></use>\n        </svg></button></th></tr>";
        for (var _i = 0, _a = templates.sort(function (a, b) { return a.name.localeCompare(b.name); }); _i < _a.length; _i++) {
            var template = _a[_i];
            var data = "data-env=\"" + template.env + "\" data-name=\"" + template.name + "\"";
            var lastRequest = template.lastRequest == 0 ? "" : (new Date(template.lastRequest)).toISOString();
            result += "<tr><td>".concat(template.env, "</td><td>").concat(template.name, "</td><td>").concat(template.created, "</td><td>").concat(template.updated, "</td><td>").concat(lastRequest, "</td>\n            <td><div class=\"btn-group\">\n            <button class=\"btn btn-sm btn-success\" ") + data + " onclick=\"Actions.showYaml(this)\"><svg class=\"bi\" width=\"1em\" height=\"1em\" fill=\"currentColor\">\n            <use xlink:href=\"bootstrap-icons.svg#eye\"></use>\n            </svg></button>\n            <button class=\"btn btn-sm btn-primary\" " + data + " onclick=\"Actions.editTemplate(this)\"><svg class=\"bi\" width=\"1em\" height=\"1em\" fill=\"currentColor\">\n            <use xlink:href=\"bootstrap-icons.svg#pen\"></use>\n            </svg></button>\n            <button class=\"btn btn-sm btn-danger\" " + data + " onclick=\"Actions.deleteTemplate(this)\"><svg class=\"bi\" width=\"1em\" height=\"1em\" fill=\"currentColor\">\n            <use xlink:href=\"bootstrap-icons.svg#eraser-fill\"></use>\n            </svg></button>\n            </div></td></tr>";
        }
        return result + "</table><h3>Total:".concat(templates.length, "</h3>");
    };
    HtmlMain.generateSecretesContent = function (secrets) {
        var result = "<table class=\"table table-striped\"><tr><th>Used</th><th></th><th>Name</th><th>Show</th><th>Created</th><th>Updated</th><th><button class=\"btn btn-sm btn-primary\" onclick=\"Actions.addSecret()\"><svg class=\"bi\" width=\"1em\" height=\"1em\" fill=\"currentColor\">\n        <use xlink:href=\"bootstrap-icons.svg#plus-circle-dotted\"></use>\n        </svg></button></th></tr>";
        for (var _i = 0, secrets_1 = secrets; _i < secrets_1.length; _i++) {
            var secret = secrets_1[_i];
            var data = "data-name=\"" + secret.name + "\"";
            var secretsAmount;
            var bg = "";
            var deleteCommandButtonStyle = "";
            var deleteAttrs = "";
            var usedAmount = secret.templatesAmount + secret.secretsAmount;
            if (usedAmount == 0) {
                bg = ' style="background: #ff000017;"';
                secretsAmount = "<span class=\"badge badge-success\" style=\"background: red;\">0</span><span class=\"badge badge-success\" style=\"background: red;\">0</span>";
                deleteCommandButtonStyle = "btn-danger";
                deleteAttrs = "onclick=\"Actions.deleteSecret(this)\"";
            }
            else {
                secretsAmount = "<span class=\"badge badge-success\" ".concat(data, " style=\"background: green;cursor:pointer;\" onclick=\"Actions.showSecretUsage(this)\">").concat(secret.templatesAmount, "</span><span class=\"badge badge-success\" ").concat(data, " style=\"background: green;cursor:pointer;\">").concat(secret.secretsAmount, "</span>");
                deleteCommandButtonStyle = "btn-default";
                deleteAttrs = "disabled=\"disabled\"";
            }
            result += "<tr" + bg + "><td>".concat(secretsAmount, "</td><td>").concat(secret.level, "</td><td>").concat(secret.name, "</td><td id=\"secret-value-") + secret.name + "\"><div style=\"cursor:pointer\" " + data + " onclick=\"Actions.showSecretValue(this)\">***</div></td><td>".concat(secret.created, "</td><td>").concat(secret.updated, "</td>\n            <td><div class=\"btn-group\"><button class=\"btn btn-sm btn-primary\" ") + data + " onclick=\"Actions.editSecret(this)\"><svg class=\"bi\" width=\"1em\" height=\"1em\" fill=\"currentColor\">\n            <use xlink:href=\"bootstrap-icons.svg#pen\"></use>\n            </svg></button><button class=\"btn btn-sm " + deleteCommandButtonStyle + "\" " + data + " " + deleteAttrs + "><svg class=\"bi\" width=\"1em\" height=\"1em\" fill=\"currentColor\">\n            <use xlink:href=\"bootstrap-icons.svg#eraser-fill\"></use>\n            </svg></button></div></td></tr>";
        }
        return result + "</table><h3>Total:".concat(secrets.length, "</h3>");
    };
    return HtmlMain;
}());
//# sourceMappingURL=HtmlMain.js.map