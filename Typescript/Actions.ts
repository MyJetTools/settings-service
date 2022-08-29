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

        let dialog = new EditTemlateDialog("Add template", undefined);
        Dialog.show(dialog);
    }

    public static editTemplate(el: HTMLElement) {
        let env = el.getAttribute('data-env');
        let name = el.getAttribute('data-name');

        let data = { env, name };

        $.ajax({ type: "POST", url: "/api/templates/get", data })
            .then(data => {
                let dialog = new EditTemlateDialog("Edit template", { env, name, yaml: data });
                Dialog.show(dialog);
            })
            .fail(() => {

            });
    }

    public static addSecret() {
        let dialog = new EditSecretDialog("Add secret", undefined);
        Dialog.show(dialog);
    }

    public static editSecret(el: HTMLElement) {
        let name = el.getAttribute('data-name');

        let data = { name };

        $.ajax({ type: "POST", url: "/api/secrets/get", data })
            .then(data => {
                let dialog = new EditSecretDialog("Edit secret", { name, secret: data });
                Dialog.show(dialog);
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



