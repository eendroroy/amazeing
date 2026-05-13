mod amazeing_context;
mod args;
mod colors;

pub(crate) mod args_impl;

pub(crate) use amazeing_context::{AmazeingContext, ContextType};
pub(crate) use args::{
    AmazeingArgs, ArgCommand, ArgHeuristic, ArgProcedure, ArgUnitShape, ArgWeightDirection, CreateArgs, SolveArgs,
    ViewArgs,
};
pub(crate) use colors::{ColorScheme, Colors};
