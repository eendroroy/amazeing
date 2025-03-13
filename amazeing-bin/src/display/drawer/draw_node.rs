use crate::context::{COLOR_CTX, DRAW_CTX};
use amazeing::DNode;
use macroquad::color::Color;
use macroquad::prelude::draw_rectangle;

pub(crate) fn draw_node(node: DNode, color: Color) {
    let ctx = DRAW_CTX.read().unwrap();
    draw_rectangle(
        ctx.margin + node.1 as f32 * (ctx.cell_width + ctx.padding),
        ctx.margin + node.0 as f32 * (ctx.cell_height + ctx.padding),
        ctx.cell_width,
        ctx.cell_height,
        color,
    );
}

pub(crate) fn draw_source(node: DNode) {
    draw_node(node, COLOR_CTX.read().unwrap().color_source);
}

pub(crate) fn draw_destination(node: DNode) {
    draw_node(node, COLOR_CTX.read().unwrap().color_destination);
}
