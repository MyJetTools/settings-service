interface ITemplate {
    env: string
    name: string,
    created: string,
    updated: string,
    lastRequest: number,
}


interface ISecret {
    templatesAmount: number,
    secretsAmount: number,
    name: string,
    created: string,
    updated: string,
    level: number,
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

interface IEditSecretDialogModel {
    name: string,
    secret: string,
    level: number,
}

interface ISecretValue {
    value: string,
    level: number,
}

interface IDeleteSecretModel {
    name: string,
}

interface ISecretModel {
    env: string,
    name: string,
    yaml: string
}