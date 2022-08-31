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

        let dialog = new EditTemlateDialog("Add template");
        Dialog.show(dialog);
        Dialog.populateData(undefined);
    }

    public static editTemplate(el: HTMLElement) {
        let env = el.getAttribute('data-env');
        let name = el.getAttribute('data-name');

        let data = { env, name };

        let dialog = new EditTemlateDialog("Edit template");
        Dialog.show(dialog);
        $.ajax({ type: "POST", url: "/api/templates/get", data })
            .then(data => {
                Dialog.populateData({ env, name, yaml: data });
            })
            .fail(() => {

            });
    }

    public static previewTemplate(el: HTMLElement) {
        let env = el.getAttribute('data-env');
        let name = el.getAttribute('data-name');

        let data = { env, name };
        let dialog = new EditTemlateDialog("Edit template");
        Dialog.show(dialog);

        $.ajax({ type: "POST", url: "/api/templates/get", data })
            .then(data => {
                Dialog.populateData({ env, name, yaml: data });
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
        Dialog.show(undefined);
    }

    public static editSecret(el: HTMLElement) {
        let name = el.getAttribute('data-name');

        let data = { name };

        let dialog = new EditSecretDialog("Edit secret");
        Dialog.show(dialog);

        $.ajax({ type: "POST", url: "/api/secrets/get", data })
            .then(data => {
                Dialog.populateData({ name, secret: data });
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



