interface IDialog {
    getOkBtnName(): string;
    getWidth(): string;
    title: string;
    getContent(): string;
    populate(mode: any): void;
    check(): any;
    ok(data: any): void,
}
