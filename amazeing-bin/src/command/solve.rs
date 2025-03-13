use crate::context::SIM_CTX;
use crate::helper::run_algorithm;
use crate::ui;
use amazeing::DNode;

pub(crate) fn solve(simulate: bool) {
    let mut tracer: Option<Vec<Vec<DNode>>> = Some(vec![]);

    run_algorithm(
        &SIM_CTX.read().unwrap().maze,
        SIM_CTX.read().unwrap().source,
        SIM_CTX.read().unwrap().destination,
        SIM_CTX.read().unwrap().proc.clone(),
        Some(SIM_CTX.read().unwrap().heuristic.clone()),
        &mut tracer,
    );

    SIM_CTX.write().unwrap().tracer = tracer.unwrap();

    if simulate {
        ui::solve_simulate::main();
    } else {
        ui::solve::main()
    }
}
