class Actions {
    public static menuClicked(e: HTMLElement) {
        let menuItem = e.getAttribute('data-itm');

        if (AppContext.selectedMenu) {
            AppContext.selectedMenu.classList.remove("menu-item-selected");
        }

        AppContext.selectedMenu = e;
        e.classList.add("menu-item-selected");

        if (menuItem == "Secrets") {
            this.loadSecrets();
        }
        else
            if (menuItem == "Templates") {
                this.loadTemplates()

            }
            else if (menuItem == "SubTemplates") {

            }
            else {
                console.log("Unknown menu item " + menuItem);
            }
    }

    public static addTemplate() {

        let dialog = new EditTemplateDialog("Add template");
        Dialog.show(dialog);
        Dialog.populateData(undefined);
    }

    public static editTemplate(el: HTMLElement) {
        let env = el.getAttribute('data-env');
        let name = el.getAttribute('data-name');

        let data = { env, name };

        let dialog = new EditTemplateDialog("Edit template");
        Dialog.show(dialog);
        $.ajax({ type: "POST", url: "/api/templates/get", data })
            .then(data => {
                Dialog.populateData({ env, name, yaml: data });
            })
            .fail(() => {

            });
    }

    public static showYaml(el: HTMLElement) {
        let env = el.getAttribute('data-env');
        let name = el.getAttribute('data-name');
        let url = Utils.compileSettingsUrl(env, name);
        let dialog = new ShowYamlDialog(url);
        Dialog.show(dialog);

        $.ajax({ type: "POST", url })
            .then(data => {
                Dialog.populateData(data);
            })
            .fail(() => {

            });
    }

    ////////////////////////////////////////////////////////////////////////////////

    public static deleteTemplate(el: HTMLElement) {
        let env = el.getAttribute('data-env');
        let name = el.getAttribute('data-name');

        let dialog = new ConfirmDeleteTemplate("Confirmation");
        Dialog.show(dialog);
        Dialog.populateData({ env, name });
    }

    public static addSecret() {
        let dialog = new EditSecretDialog("Add secret");
        Dialog.show(dialog);
        Dialog.populateData(undefined);
    }

    public static editSecret(el: HTMLElement) {
        let name = el.getAttribute('data-name');

        let data = { name };

        let dialog = new EditSecretDialog("Edit secret");
        Dialog.show(dialog);

        $.ajax({ type: "POST", url: "/api/secrets/get", data })
            .then((data: ISecretValue) => {
                let value: IEditSecretDialogModel = {
                    name, secret: data.value, level: data.level
                };
                Dialog.populateData(value);
            })
            .fail(() => {

            });
    }

    public static deleteSecret(el: HTMLElement) {
        let name = el.getAttribute('data-name');

        let dialog = new ConfirmDeleteSecret("Confirmation");
        Dialog.show(dialog);
        Dialog.populateData({ name });
    }

    public static showSecretUsage(el: HTMLElement) {
        let name = el.getAttribute('data-name');
        let data = { name };
        let dialog = new ShowSecretUsageDialog("Show secret [" + name + "] usage", name);
        Dialog.show(dialog);

        $.ajax({ type: "POST", url: "/api/secrets/usage", data })
            .then(data => {
                Dialog.populateData(data.data);
            })
            .fail(() => {

            });
        Dialog.populateData({ name });
    }

    public static showSecretValue(el: HTMLElement) {
        let name = el.getAttribute('data-name');

        let elToUpdate = document.getElementById("secret-value-" + name);

        elToUpdate.innerHTML = '<img src="/img/loading.gif" style="height:32px"></img>';

        let data = { name };

        $.ajax({ type: "POST", url: "/api/secrets/show", data })
            .then((data) => {
                elToUpdate.innerHTML = data;
            })
            .fail(() => {

            });
    }



    /// Operation

    public static loadTemplates() {

        $.ajax({ method: "POST", url: "/api/templates/getall" })
            .then(data => {
                AppContext.contentElement.innerHTML = HtmlMain.generateTemplateContent(data.data);
            })
            .fail(() => {

            });

    }

    public static loadSecrets() {

        $.ajax({ method: "POST", url: "/api/secrets/getall" })
            .then(data => {
                AppContext.contentElement.innerHTML = HtmlMain.generateSecretesContent(data.data);
            })
            .fail(() => {

            });

    }



}



