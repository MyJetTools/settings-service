class EditTemlateDialog implements IDialog {
    title: string;
    getWidth(): string {
        return undefined;
    }

    edtEnv: HTMLInputElement
    edtName: HTMLInputElement
    edtYaml: HTMLInputElement

    getContent() {
        return `
        <div class="form-floating mb-3">
        <input class="form-control" id="edtEnv" name="env">
        <label for="edtEnv">Env</label>
        </div>

        <div class="form-floating mb-3">
        <input class="form-control" id="edtName" name="name">
        <label for="edtName">Name</label>
        </div>

        <div class="form-floating">
        <textarea class="form-control" style="min-height:500px;font-family: monospace;" id="edtYaml"  name="yaml"></textarea>
        <label for="edtYaml">Yaml</label>
        </div>`;
    }

    populate() {
        this.edtEnv = document.getElementById('edtEnv') as HTMLInputElement;
        this.edtName = document.getElementById('edtName') as HTMLInputElement;
        this.edtYaml = document.getElementById('edtYaml') as HTMLInputElement;
        if (this.viewModel) {
            this.edtEnv.value = this.viewModel.env;
            this.edtEnv.readOnly = true;
            this.edtName.value = this.viewModel.name;
            this.edtName.readOnly = true;
            this.edtYaml.value = this.viewModel.yaml;
        }
    };

    check(): IEditTemplateModel {
        if (!passElement(this.edtEnv)) {
            return undefined;
        }

        if (!passElement(this.edtName)) {
            return undefined;
        }

        if (!passElement(this.edtYaml)) {
            return undefined;
        }

        return {
            env: this.edtEnv.value,
            name: this.edtName.value,
            yaml: this.edtYaml.value
        }
    }

    public ok(data: IEditTemplateModel) {
        console.log(data);
    }

    viewModel: IEditTemplateModel;

    constructor(title: string, viewModel: IEditTemplateModel) {
        this.title = title;
        this.viewModel = viewModel;
    }
}


function passElement(edt: HTMLInputElement): boolean {
    console.log(edt.id);
    console.log(edt.value);
    if (edt.value == "") {
        edt.focus();
        edt.select()
        return false;
    }

    return true
}