
class Dialog {

  private static dialogData: IDialog;

  public static show(data: IDialog) {
    AppContext.dialogPadElement.classList.remove('dialog-pad-hidden');
    AppContext.menuElement.classList.add('blur-content');
    AppContext.contentElement.classList.add('blur-content');

    AppContext.dialogPadElement.innerHTML = this.generateDialog(data);
  }

  public static populateData(model: any) {
    let el = document.getElementById('modal-content');
    el.innerHTML = this.dialogData.getContent();
    this.dialogData.populate(model);
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
              <button type="button" class="btn btn-default btn-sm" data-bs-dismiss="modal" aria-label="Close" onclick="Dialog.hide()"><svg class="bi" width="1em" height="1em" fill="currentColor">
              <use xlink:href="bootstrap-icons.svg#x"></use>
              </svg></button>
            </div>
            <div id="modal-content" class="modal-body">
                <div style="text-align:center;"><img src="/img/loading.gif" style="width: 90px;" /></div>
            </div>
            <div class="modal-footer">
            <div class="btn-group">
            <button type="button" class="btn btn-primary  btn-sm" onclick="Dialog.onOkPressed()"><svg class="bi" width="1em" height="1em" fill="currentColor">
            <use xlink:href="bootstrap-icons.svg#check"></use>
            </svg>`+ data.getOkBtnName() + `</button>
              <button type="button" class="btn btn-secondary btn-sm" data-bs-dismiss="modal" onclick="Dialog.hide()"><svg class="bi" width="1em" height="1em" fill="currentColor">
              <use xlink:href="bootstrap-icons.svg#x"></use>
              </svg>Cancel</button>
            </div>
            </div>
          </div>
        </div>`;
  }
}