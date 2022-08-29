class ConfirmDeleteTemplate implements IDialog {

    title: string;

    getWidth(): string {
        return "600px";
    }

    getOkBtnName(): string {
        return "Confirm";
    }

    getContent() {
        return `<h4>You are about to delete template</h4>`;
    }

    populate() {
    }

    check(): any {
        return true;
    }

    public ok(_: any) {
        $.ajax({ type: "POST", url: "/api/templates/delete", data: this.data })
            .then(() => {
                Actions.loadTemplates();
            })
            .fail(() => {

            });
    }

    data: IDeleteTemplateModel;

    constructor(title: string, data: IDeleteTemplateModel) {
        this.title = title;
        this.data = data;
    }
}