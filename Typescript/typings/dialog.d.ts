interface IDialog {
    getOkBtnName(): string;
    getWidth(): string;
    title: string;
    getContent(): string;
    populate(): void;
    check(): any;
    ok(data: any): void,
}
