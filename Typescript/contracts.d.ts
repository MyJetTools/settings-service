interface ITemplate {
    env: string
    name: string,
    created: string,
    updated: string,
}


interface ISecret {
    name: string,
    created: string,
    updated: string,
}

interface IEditTemplateModel {
    env: string
    name: string,
    yaml: string,
}

interface IDeleteTemplateModel {
    env: string
    name: string,
}

interface IEditSecret {
    name: string,
    secret: string,
}

interface IDeleteSecretModel {
    name: string,
}