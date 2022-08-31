class ShowYamlDialog implements IDialog {

    title: string;

    hideCancelBtn = true;


    getWidth(): string {
        return "90%";
    }

    getOkBtnName(): string {
        return "Ok";
    }

    getContent() {
        return `<div class="form-floating">
        <textarea class="form-control" style="min-height:500px;font-family: monospace;" id="showYaml" readonly="readonly"></textarea>
        <label for="edtYaml">Yaml</label>
        </div>`;
    }

    populate(data: string) {
        document.getElementById('showYaml').innerHTML = data;
    }

    check(): any {
        return true;
    }

    public ok(_: any) {

    }

    data: IDeleteSecretModel;

    constructor(title: string) {
        this.title = title;

    }
}