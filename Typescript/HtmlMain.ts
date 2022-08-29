class HtmlMain {
    public static generateLayout(): string {

        return `<div id="content"></div><div id="menu-bar" style="width:200px;">` + this.generateStatusBar() +
            `</div><div id="dialog-pad" class="dialog-pad-hidden"></div>`;

    }


    static generateStatusBar(): string {
        return `<div class="title"> <h3>Settings </h3></div> `
            + `<div class="menu-item" data-itm="Secrets" onclick="Actions.menuClicked(this)"> Secrets </div>`
            + `<div class="menu-item" data-itm="Templates" onclick="Actions.menuClicked(this)">Templates</div>`
            + `<div class="menu-item" data-itm="SubTemplates" onclick="Actions.menuClicked(this)">Sub templates</div>`;
    }


    public static generateTemplateContent(templates: ITemplate[]): string {

        let result = `<table class="table table-striped"><tr><th>Env</th><th>Name</th><th>Created</th><th>Updated</th><th><button class="btn btn-sm btn-primary" onclick="Actions.addTemplate()">Add</button></th></tr>`;

        for (let template of templates) {
            result += `<tr><td>${template.env}</td><td>${template.name}</td><td>${template.created}</td><td>${template.updated}</td>
            <td><button class="btn btn-sm btn-primary" data-env="`+ template.env + `" data-name="` + template.name + `" onclick="Actions.editTemplate(this)">Edit</button><button class="btn btn-sm btn-danger">Delete</button></td></tr>`;
        }

        return result + "</table>";
    }

    public static generateSecretesContent(secrets: ISecret[]): string {

        let result = `<table class="table table-striped"><tr><th>Name</th><th>Created</th><th>Updated</th><th><button class="btn btn-sm btn-primary" onclick="Actions.addSecret()">Add</button></th></tr>`;

        for (let secret of secrets) {
            result += `<tr><td>${secret.name}</td><td>${secret.created}</td><td>${secret.updated}</td>
            <td><button class="btn btn-sm btn-primary"  data-name="` + secret.name + `" onclick="Actions.editSecret(this)">Edit</button><button class="btn btn-sm btn-danger">Delete</button></td></tr>`;
        }

        return result + "</table>";
    }


}