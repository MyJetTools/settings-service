class HtmlMain {
    public static generateLayout(): string {
        return `<div id="content"></div><div id="menu-bar" style="width:200px;">` + this.generateStatusBar() +
            `</div><div id="dialog-pad" class="dialog-pad-hidden"></div>`;
    }

    static generateStatusBar(): string {
        let env = document.body.getAttribute("data-env");
        let envColor = document.body.getAttribute("data-env-color");
        return `<div class="title"> <h3>Settings </h3><h4 style="text-shadow: 1px 1px 1px black;color: ` + envColor + `">` + env + `</h4></div> `
            + `<div class="menu-item" data-itm="Secrets" onclick="Actions.menuClicked(this)"> Secrets </div>`
            + `<div class="menu-item" data-itm="Templates" onclick="Actions.menuClicked(this)">Templates</div>`
            + `<div class="menu-item" data-itm="SubTemplates" onclick="Actions.menuClicked(this)">Sub templates</div>`;
    }

    public static generateTemplateContent(templates: ITemplate[]): string {
        let result = `<table class="table table-striped"><tr><th>Env</th><th>Name</th><th>Created</th><th>Updated</th><th>LastRequest</th><th><button class="btn btn-sm btn-primary" onclick="Actions.addTemplate()"><svg class="bi" width="1em" height="1em" fill="currentColor">
        <use xlink:href="bootstrap-icons.svg#plus-circle-dotted"></use>
        </svg></button></th></tr>`;
        for (let template of templates.sort((a, b) => a.name.localeCompare(b.name))) {
            let data = `data-env="` + template.env + `" data-name="` + template.name + `"`;

            let lastRequest = template.lastRequest == 0 ? "" : (new Date(template.lastRequest)).toISOString();
            result += `<tr><td>${template.env}</td><td>${template.name}</td><td>${template.created}</td><td>${template.updated}</td><td>${lastRequest}</td>
            <td><div class="btn-group">
            <button class="btn btn-sm btn-success" `+ data + ` onclick="Actions.showYaml(this)"><svg class="bi" width="1em" height="1em" fill="currentColor">
            <use xlink:href="bootstrap-icons.svg#eye"></use>
            </svg></button>
            <button class="btn btn-sm btn-primary" `+ data + ` onclick="Actions.editTemplate(this)"><svg class="bi" width="1em" height="1em" fill="currentColor">
            <use xlink:href="bootstrap-icons.svg#pen"></use>
            </svg></button>
            <button class="btn btn-sm btn-danger" ` + data + ` onclick="Actions.deleteTemplate(this)"><svg class="bi" width="1em" height="1em" fill="currentColor">
            <use xlink:href="bootstrap-icons.svg#eraser-fill"></use>
            </svg></button>
            </div></td></tr>`;
        }
        return result + `</table><h3>Total:${templates.length}</h3>`;
    }

    public static generateSecretesContent(secrets: ISecret[]): string {
        let result = `<table class="table table-striped"><tr><th>Used</th><th>Name</th><th>Show</th><th>Created</th><th>Updated</th><th><button class="btn btn-sm btn-primary" onclick="Actions.addSecret()"><svg class="bi" width="1em" height="1em" fill="currentColor">
        <use xlink:href="bootstrap-icons.svg#plus-circle-dotted"></use>
        </svg></button></th></tr>`;
        for (let secret of secrets) {
            let data = `data-name="` + secret.name + `"`;

            var secretsAmount;
            var bg = "";
            var deleteCommandButtonStyle = "";
            var deleteAttrs = "";

            if (secret.amount > 0) {
                secretsAmount = `<span class="badge badge-success" ` + data + ` style="background: green;cursor:pointer;" onclick="Actions.showSecretUsage(this)">${secret.amount}</span>`;
                deleteCommandButtonStyle = "btn-default";
                deleteAttrs = `disabled="disabled"`;
            }
            else {
                bg = ' style="background: #ff000017;"';
                secretsAmount = `<span class="badge badge-success" style="background: red;">0</span>`;
                deleteCommandButtonStyle = "btn-danger";
                deleteAttrs = `onclick="Actions.deleteSecret(this)"`;
            }


            result += `<tr` + bg + `><td>` + secretsAmount + `</td><td>${secret.name}</td><td id="secret-value-` + secret.name + `"><div style="cursor:pointer" ` + data + ` onclick="Actions.showSecretValue(this)">***</div></td><td>${secret.created}</td><td>${secret.updated}</td>
            <td><div class="btn-group"><button class="btn btn-sm btn-primary" ` + data + ` onclick="Actions.editSecret(this)"><svg class="bi" width="1em" height="1em" fill="currentColor">
            <use xlink:href="bootstrap-icons.svg#pen"></use>
            </svg></button><button class="btn btn-sm `+ deleteCommandButtonStyle + `" ` + data + ` ` + deleteAttrs + `><svg class="bi" width="1em" height="1em" fill="currentColor">
            <use xlink:href="bootstrap-icons.svg#eraser-fill"></use>
            </svg></button></div></td></tr>`;
        }
        return result + `</table><h3>Total:${secrets.length}</h3>`;
    }
}