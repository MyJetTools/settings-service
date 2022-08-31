
class EditSecretDialog implements IDialog {
    title: string;

    hideCancelBtn = false;

    getWidth(): string {
        return "600px";
    }

    getOkBtnName(): string {
        return "Save";
    }

    edtName: HTMLInputElement
    edtSecret: HTMLInputElement

    getContent() {
        return `
        <div class="form-floating mb-3">
        <input class="form-control" id="edtName" name="name">
        <label for="edtName">Name</label>
        </div>

      <div class="form-floating">
      <input class="form-control" id="edtSecret" name="secret">
      <label for="edtSecret">Secret</label>
      </div>`;
    }


    populate(viewModel: IEditSecret) {
        this.viewModel = viewModel;
        this.edtName = document.getElementById('edtName') as HTMLInputElement;
        this.edtSecret = document.getElementById('edtSecret') as HTMLInputElement;
        if (this.viewModel) {
            this.edtName.value = this.viewModel.name;
            this.edtName.readOnly = true;
            this.edtSecret.value = this.viewModel.secret;
        }
    };

    check(): IEditSecret {

        if (!passElement(this.edtName)) {
            return undefined;
        }

        if (!passElement(this.edtSecret)) {
            return undefined;
        }

        return {
            name: this.edtName.value,
            secret: this.edtSecret.value
        }
    }

    public ok(data: IEditSecret) {
        $.ajax({ type: "POST", url: "/api/secrets/post", data })
            .then(() => {
                Actions.loadSecrets();
            })
            .fail(() => {

            });
    }

    viewModel: IEditSecret;

    constructor(title: string) {
        this.title = title;
    }

}