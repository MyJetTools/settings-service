var HtmlMain = /** @class */ (function () {
    function HtmlMain() {
    }
    HtmlMain.generateLayout = function () {
        return "<div id=\"content\"></div><div id=\"menu-bar\" style=\"width:200px;\">" + this.generateStatusBar() +
            "</div><div id=\"dialog-pad\" class=\"dialog-pad-hidden\"></div>";
    };
    HtmlMain.generateStatusBar = function () {
        return "<div class=\"title\"> <h3>Settings </h3></div> "
            + "<div class=\"menu-item\" data-itm=\"Secrets\" onclick=\"Actions.menuClicked(this)\"> Secrets </div>"
            + "<div class=\"menu-item\" data-itm=\"Templates\" onclick=\"Actions.menuClicked(this)\">Templates</div>"
            + "<div class=\"menu-item\" data-itm=\"SubTemplates\" onclick=\"Actions.menuClicked(this)\">Sub templates</div>";
    };
    HtmlMain.generateTemplateContent = function (templates) {
        var result = "<table class=\"table table-striped\"><tr><th>Env</th><th>Name</th><th>Created</th><th>Updated</th><th><button class=\"btn btn-sm btn-primary\" onclick=\"Actions.addTemplate()\">Add</button></th></tr>";
        for (var _i = 0, templates_1 = templates; _i < templates_1.length; _i++) {
            var template = templates_1[_i];
            var data = "data-env=\"" + template.env + "\" data-name=\"" + template.name + "\"";
            result += "<tr><td>".concat(template.env, "</td><td>").concat(template.name, "</td><td>").concat(template.created, "</td><td>").concat(template.updated, "</td>\n            <td><div class=\"btn-group\"><button class=\"btn btn-sm btn-primary\" ") + data + " onclick=\"Actions.editTemplate(this)\">Edit</button><button class=\"btn btn-sm btn-danger\" " + data + " onclick=\"Actions.deleteTemplate(this)\">Delete</button></div></td></tr>";
        }
        return result + "</table>";
    };
    HtmlMain.generateSecretesContent = function (secrets) {
        var result = "<table class=\"table table-striped\"><tr><th>Name</th><th>Created</th><th>Updated</th><th><button class=\"btn btn-sm btn-primary\" onclick=\"Actions.addSecret()\">Add</button></th></tr>";
        for (var _i = 0, secrets_1 = secrets; _i < secrets_1.length; _i++) {
            var secret = secrets_1[_i];
            result += "<tr><td>".concat(secret.name, "</td><td>").concat(secret.created, "</td><td>").concat(secret.updated, "</td>\n            <td><div class=\"btn-group\"><button class=\"btn btn-sm btn-primary\"  data-name=\"") + secret.name + "\" onclick=\"Actions.editSecret(this)\">Edit</button><button class=\"btn btn-sm btn-danger\">Delete</button></div></td></tr>";
        }
        return result + "</table>";
    };
    return HtmlMain;
}());
//# sourceMappingURL=HtmlMain.js.map