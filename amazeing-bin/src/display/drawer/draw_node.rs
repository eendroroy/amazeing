use crate::context::{DrawContext, CONTEXT};
use amazeing::DNode;
use macroquad::color::Color;
use macroquad::prelude::draw_rectangle;

pub(crate) fn draw_node(node: DNode, ctx: &DrawContext, color: Color) {
    draw_rectangle(
        ctx.margin + node.1 as f32 * (ctx.cell_width + ctx.padding),
        ctx.margin + node.0 as f32 * (ctx.cell_height + ctx.padding),
        ctx.cell_width,
        ctx.cell_height,
        color,
    );
}

pub(crate) fn draw_source(node: DNode, ctx: &DrawContext) {
    draw_node(node, ctx, CONTEXT.read().unwrap().colors.color_source);
}

pub(crate) fn draw_destination(node: DNode, ctx: &DrawContext) {
    draw_node(node, ctx, CONTEXT.read().unwrap().colors.color_destination);
}

pub(crate) fn draw_source_destination(source: DNode, destination: DNode, ctx: &DrawContext) {
    draw_source(source, ctx);
    draw_destination(destination, ctx);
}
