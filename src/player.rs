use crate::CombatStats;

use super::{Map, Player, Position, RunState, State, Viewshed};
use rltk::{console, Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

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

    fn up_right() -> Move {
        Move {
            delta_x: 1,
            delta_y: -1,
        }
    }

    fn up_left() -> Move {
        Move {
            delta_x: -1,
            delta_y: -1,
        }
    }

    fn down_right() -> Move {
        Move {
            delta_x: 1,
            delta_y: 1,
        }
    }

    fn down_left() -> Move {
        Move {
            delta_x: -1,
            delta_y: 1,
        }
    }
}

fn try_move_player(mv: Move, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let destination_idx = map.xy_idx(pos.x + mv.delta_x, pos.y + mv.delta_y);

        for potential_target in map.tile_content[destination_idx].iter() {
            let target = combat_stats.get(*potential_target);
            match target {
                None => {}
                Some(_t) => {
                    console::log(&format!("From Hell's Heard, I stab thee!"));
                    return;
                }
            }
        }

        if !map.blocked[destination_idx] {
            pos.x = min(map.width - 1, max(0, pos.x + mv.delta_x));
            pos.y = min(map.height - 1, max(0, pos.y + mv.delta_y));

            let mut ppos = ecs.write_resource::<rltk::Point>();
            ppos.x = pos.x;
            ppos.y = pos.y;

            viewshed.dirty = true;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => return RunState::Paused,
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => {
                try_move_player(Move::left(), &mut gs.ecs)
            }

            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => {
                try_move_player(Move::right(), &mut gs.ecs)
            }

            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => {
                try_move_player(Move::up(), &mut gs.ecs)
            }

            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => {
                try_move_player(Move::down(), &mut gs.ecs)
            }

            VirtualKeyCode::Numpad9 | VirtualKeyCode::Y => {
                try_move_player(Move::up_right(), &mut gs.ecs)
            }
            VirtualKeyCode::Numpad7 | VirtualKeyCode::U => {
                try_move_player(Move::up_left(), &mut gs.ecs)
            }
            VirtualKeyCode::Numpad3 | VirtualKeyCode::N => {
                try_move_player(Move::down_right(), &mut gs.ecs)
            }
            VirtualKeyCode::Numpad1 | VirtualKeyCode::B => {
                try_move_player(Move::down_left(), &mut gs.ecs)
            }

            _ => return RunState::Paused,
        },
    }

    RunState::Running
}
