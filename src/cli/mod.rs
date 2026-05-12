mod amazeing_context;
mod args;

pub(crate) mod args_impl;

pub(crate) use amazeing_context::{AmazeingContext, ColorScheme, Colors, ContextType};
pub(crate) use args::{
	AmazeingArgs, ArgCommand, ArgHeuristic, ArgProcedure, ArgUnitShape, ArgWeightDirection, CreateArgs, SolveArgs,
	ViewArgs,
};
