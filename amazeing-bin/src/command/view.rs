use crate::ui;

pub(crate) fn view(modify: bool) {
    if modify {
        ui::update::main()
    } else {
        ui::view::main();
    }
}
