
class Dialog {

  private static dialogData: IDialog;

  public static show(data: IDialog) {
    AppContext.dialogPadElement.classList.remove('dialog-pad-hidden');
    AppContext.menuElement.classList.add('blur-content');
    AppContext.contentElement.classList.add('blur-content');

    AppContext.dialogPadElement.innerHTML = this.generateDialog(data);

    data.populate();
  }

  public static hide() {

    AppContext.dialogPadElement.classList.add('dialog-pad-hidden');
    AppContext.menuElement.classList.remove('blur-content');
    AppContext.contentElement.classList.remove('blur-content');

  }


  static onOkPressed() {
    let result = this.dialogData.check();
    if (result) {
      this.hide();
      this.dialogData.ok(result);
    }
  }


  static generateDialog(data: IDialog) {
    this.dialogData = data;

    let width = data.getWidth();
    if (width == undefined) {
      width = "";
    }
    else {
      width = `style="width: ${width}"`;
    }

    return `
        <div class="modal-dialog" `+ width + `>
          <div class="modal-content">
            <div class="modal-header">
              <h5 class="modal-title">`+ data.title + `</h5>
              <button type="button" class="btn btn-default" data-bs-dismiss="modal" aria-label="Close" onclick="Dialog.hide()">X</button>
            </div>
            <div class="modal-body">
                ` + data.getContent() + `
            </div>
            <div class="modal-footer">
            <div class="btn-group">
            <button type="button" class="btn btn-primary" onclick="Dialog.onOkPressed()">`+ data.getOkBtnName() + `</button>
              <button type="button" class="btn btn-secondary" data-bs-dismiss="modal" onclick="Dialog.hide()">Cancel</button>
            </div>
            </div>
          </div>
        </div>`;
  }
}