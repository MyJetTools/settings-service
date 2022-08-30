class ConfirmDeleteSecret implements IDialog {

    title: string;

    getWidth(): string {
        return "600px";
    }

    getOkBtnName(): string {
        return "Confirm";
    }

    getContent() {
        return `<h4>You are about to delete secret</h4>`;
    }

    populate() {
    }

    check(): any {
        return true;
    }

    public ok(_: any) {
        $.ajax({ type: "POST", url: "/api/secrets/delete", data: this.data })
            .then(() => {
                Actions.loadTemplates();
            })
            .fail(() => {

            });
    }

    data: IDeleteSecretModel;

    constructor(title: string, data: IDeleteSecretModel) {
        this.title = title;
        this.data = data;
    }
}