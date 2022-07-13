use rltk::{VirtualKeyCode, Rltk};
use specs::prelude::*;
use super::{Position, Player, TileType, xy_idx, State};
use std::cmp::{min, max};

struct Move {
    delta_x: i32,
    delta_y: i32,
}

impl Move {
    fn left() -> Move {
        Move {
            delta_x: -1,
            delta_y: 0,
        }
    }

    fn right() -> Move {
        Move {
            delta_x: 1,
            delta_y: 0,
        }
    }

    fn up() -> Move {
        Move {
            delta_x: 0,
            delta_y: -1,
        }
    }

    fn down() -> Move {
        Move {
            delta_x: 0,
            delta_y: 1,
        }
    }
}

fn try_move_player(mv: Move, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = xy_idx(pos.x + mv.delta_x, pos.y + mv.delta_y);
        if map[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + mv.delta_x));
            pos.y = min(49, max(0, pos.y + mv.delta_y));
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left |
            VirtualKeyCode::Numpad4 |
            VirtualKeyCode::H => try_move_player(Move::left(), &mut gs.ecs),

            VirtualKeyCode::Right |
            VirtualKeyCode::Numpad6 |
            VirtualKeyCode::L => try_move_player(Move::right(), &mut gs.ecs),

            VirtualKeyCode::Up |
            VirtualKeyCode::Numpad8 |
            VirtualKeyCode::K => try_move_player(Move::up(), &mut gs.ecs),

            VirtualKeyCode::Down |
            VirtualKeyCode::Numpad2 |
            VirtualKeyCode::J => try_move_player(Move::down(), &mut gs.ecs),

            _ => {}
        },
    }
}
