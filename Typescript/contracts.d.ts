interface ITemplate {
    product: string
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
    product: string
    name: string,
    yaml: string,
}

interface IDeleteTemplateModel {
    product: string
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
    product: string,
    name: string,
    yaml: string
}


interface ISecretUsage {
    name: String,
    value: String,
}