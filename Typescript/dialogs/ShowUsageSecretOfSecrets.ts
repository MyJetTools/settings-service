class ShowUsageSecretOfSecrets implements IDialog {

    title: string;

    hideCancelBtn = true;

    secretName: string;


    getWidth(): string {
        return "90%";
    }

    getOkBtnName(): string {
        return "Ok";
    }

    getContent() {
        return `
        <div class="form-control" style="height:500px;font-family: monospace; overflow-y: scroll;" id="secretUsage" readonly="readonly"></div>
        `;
    }

    populate(data: ISecretUsage[]) {

        let result = "";
        for (let itm of data) {

            result += '<div>' + itm.name + ': ' + itm.value + '</div>';
        }

        document.getElementById('secretUsage').innerHTML = result;
    }

    check(): any {
        return true;
    }

    public ok(_: any) {

    }

    data: IDeleteSecretModel;

    constructor(title: string, secretName: string) {
        this.title = title;
        this.secretName = secretName;

    }
}

