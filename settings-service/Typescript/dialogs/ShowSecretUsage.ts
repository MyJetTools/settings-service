class ShowSecretUsageDialog implements IDialog {

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

    populate(data: ISecretModel[]) {

        let result = "";
        for (let itm of data) {
            result += '<h4>' + itm.product + '/' + itm.name + '</h4>';

            for (let line of itm.yaml.split(/\r?\n/)) {

                if (line.indexOf(this.secretName) >= 0) {

                    result += '<div>';

                    for (let i = 0; i < spacesAmount(line); i++) {
                        result += '&nbsp;';
                    }

                    result += '<b>' + line.trim() + '</b></div>';
                } else {

                    result += '<div style="color:gray">';

                    for (let i = 0; i < spacesAmount(line); i++) {
                        result += '&nbsp;';
                    }
                    result += line.trim() + '</div>';
                }


            }

            result += '<hr/>';
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


function splitByLines(text: string) {
    return text.split(/\r?\n/);
}

function spacesAmount(text: string): number {
    let result = 0;

    for (let i = 0; i < text.length; i++) {
        if (text[i] == ' ') {
            result++;
        } else {
            break;
        }
    }

    return result;
}