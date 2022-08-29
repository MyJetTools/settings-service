var HtmlStaticElement = /** @class */ (function () {
    function HtmlStaticElement(el, renderHtml) {
        this.el = el;
        this.renderHtml = renderHtml;
    }
    HtmlStaticElement.prototype.update = function (value) {
        if (this.value === undefined || this.value != value) {
            this.value = value;
            this.el.innerHTML = this.renderHtml(value);
        }
    };
    return HtmlStaticElement;
}());
//# sourceMappingURL=HtmlStaticElement.js.map