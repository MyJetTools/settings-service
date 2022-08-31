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
        var dialog = new EditTemlateDialog("Add template");
        Dialog.show(dialog);
        Dialog.populateData(undefined);
    };
    Actions.editTemplate = function (el) {
        var env = el.getAttribute('data-env');
        var name = el.getAttribute('data-name');
        var data = { env: env, name: name };
        var dialog = new EditTemlateDialog("Edit template");
        Dialog.show(dialog);
        $.ajax({ type: "POST", url: "/api/templates/get", data: data })
            .then(function (data) {
            Dialog.populateData({ env: env, name: name, yaml: data });
        })
            .fail(function () {
        });
    };
    Actions.showYaml = function (el) {
        var env = el.getAttribute('data-env');
        var name = el.getAttribute('data-name');
        var url = Utils.compileSettingsUrl(env, name);
        var dialog = new ShowYamlDialog(url);
        Dialog.show(dialog);
        $.ajax({ type: "POST", url: url })
            .then(function (data) {
            Dialog.populateData(data);
        })
            .fail(function () {
        });
    };
    ////////////////////////////////////////////////////////////////////////////////
    Actions.deleteTemplate = function (el) {
        var env = el.getAttribute('data-env');
        var name = el.getAttribute('data-name');
        var dialog = new ConfirmDeleteTemplate("Confirmation");
        Dialog.show(dialog);
        Dialog.populateData({ env: env, name: name });
    };
    Actions.addSecret = function () {
        var dialog = new EditSecretDialog("Add secret");
        Dialog.show(dialog);
        Dialog.show(undefined);
    };
    Actions.editSecret = function (el) {
        var name = el.getAttribute('data-name');
        var data = { name: name };
        var dialog = new EditSecretDialog("Edit secret");
        Dialog.show(dialog);
        $.ajax({ type: "POST", url: "/api/secrets/get", data: data })
            .then(function (data) {
            Dialog.populateData({ name: name, secret: data });
        })
            .fail(function () {
        });
    };
    Actions.deleteSecret = function (el) {
        var name = el.getAttribute('data-name');
        var dialog = new ConfirmDeleteSecret("Confirmation");
        Dialog.show(dialog);
        Dialog.populateData({ name: name });
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