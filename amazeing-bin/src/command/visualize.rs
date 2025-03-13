use crate::ui;

pub(crate) fn visualize(modify: bool) {
    if modify {
        ui::modify::main()
    } else {
        ui::visualize::main();
    }
}
