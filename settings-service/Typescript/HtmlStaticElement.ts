
class HtmlStaticElement<T> {
    value: T;
    el: HTMLElement;
    renderHtml: (value: T) => string;

    constructor(el: HTMLElement, renderHtml: (value: T) => string) {
        this.el = el;
        this.renderHtml = renderHtml;
    }

    public update(value: T) {
        if (this.value === undefined || this.value != value) {
            this.value = value;
            this.el.innerHTML = this.renderHtml(value);
        }
    }

}