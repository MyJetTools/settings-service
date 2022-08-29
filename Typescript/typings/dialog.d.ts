interface IDialog {
    getWidth(): string;
    title: string;
    getContent(): string;
    populate(): void;
    check(): any;
    ok(data: any): void,
}
