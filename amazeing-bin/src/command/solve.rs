use crate::ui;

pub(crate) fn solve(simulate: bool) {
    if simulate {
        ui::solve_simulate::main();
    } else {
        ui::solve::main()
    }
}
