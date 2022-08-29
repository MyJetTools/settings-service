var Actions = /** @class */ (function () {
    function Actions() {
    }
    Actions.menuClicked = function (e) {
        var menuItem = e.getAttribute('data-itm');
        if (AppContext.selectedMenu) {
            AppContext.selectedMenu.classList.remove("menu-item-selected");
        }
        AppContext.selectedMenu = e;
        e.classList.add("menu-item-selected");
        if (menuItem == "Secrets") {
            this.loadSecrets();
        }
        else if (menuItem == "Templates") {
            this.loadTemplates();
        }
        else if (menuItem == "SubTemplates") {
        }
        else {
            console.log("Unknown menu item " + menuItem);
        }
    };
    Actions.addTemplate = function () {
        var dialog = new EditTemlateDialog("Add template", undefined);
        Dialog.show(dialog);
    };
    Actions.editTemplate = function (el) {
        var env = el.getAttribute('data-env');
        var name = el.getAttribute('data-name');
        var data = { env: env, name: name };
        $.ajax({ type: "POST", url: "/api/templates/get", data: data })
            .then(function (data) {
            var dialog = new EditTemlateDialog("Edit template", { env: env, name: name, yaml: data });
            Dialog.show(dialog);
        })
            .fail(function () {
        });
    };
    Actions.deleteTemplate = function (el) {
        var env = el.getAttribute('data-env');
        var name = el.getAttribute('data-name');
        var dialog = new ConfirmDeleteTemplate("Confirmation", { env: env, name: name });
        Dialog.show(dialog);
    };
    Actions.addSecret = function () {
        var dialog = new EditSecretDialog("Add secret", undefined);
        Dialog.show(dialog);
    };
    Actions.editSecret = function (el) {
        var name = el.getAttribute('data-name');
        var data = { name: name };
        $.ajax({ type: "POST", url: "/api/secrets/get", data: data })
            .then(function (data) {
            var dialog = new EditSecretDialog("Edit secret", { name: name, secret: data });
            Dialog.show(dialog);
        })
            .fail(function () {
        });
    };
    /// Operation
    Actions.loadTemplates = function () {
        $.ajax({ method: "POST", url: "/api/templates/getall" })
            .then(function (data) {
            AppContext.contentElement.innerHTML = HtmlMain.generateTemplateContent(data.data);
        })
            .fail(function () {
        });
    };
    Actions.loadSecrets = function () {
        $.ajax({ method: "POST", url: "/api/secrets/getall" })
            .then(function (data) {
            AppContext.contentElement.innerHTML = HtmlMain.generateSecretesContent(data.data);
        })
            .fail(function () {
        });
    };
    return Actions;
}());
//# sourceMappingURL=Actions.js.map