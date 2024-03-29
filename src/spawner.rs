use rltk::{RGB, RandomNumberGenerator};
use specs::prelude::*;
use super::{CombatStats, Player, Renderable, Name, Position, Viewshed, Monster, BlockTile};

pub fn player(ecs: &mut World, pos: Position) -> Entity {
    ecs.create_entity()
        .with(pos)
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .with(Viewshed{ visible_tiles: Vec::new(), range: 8, dirty: true })
        .with(Name { name: "Player".to_string() })
        .with(CombatStats { max_hp: 30, hp: 30, defense: 2, power: 5 })
        .build()
}

pub fn random_monster(ecs: &mut World, pos: Position) -> Entity {
    let roll: i32;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        roll = rng.roll_dice(1, 2)
    }

    match roll {
        1 => orc(ecs, pos),
        _ => goblin(ecs, pos)
    }
}

fn orc(ecs: &mut World, pos: Position) -> Entity {
    monster(ecs, pos, rltk::to_cp437('o'), "Orc")
}

fn goblin(ecs: &mut World, pos: Position) -> Entity {
    monster(ecs, pos, rltk::to_cp437('g'), "Goblin")
}

fn monster<S: ToString>(ecs: &mut World, pos: Position, glyph: rltk::FontCharType, name: S) -> Entity {
    ecs.create_entity()
        .with(pos)
        .with(Renderable{
            glyph,
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rlkt::BLACK),
        })
        .with(Viewshed{visible_tiles: Vec::new(), range: 8, dirty: true})
        .with(Monster{})
        .with(Name{name: name.to_string()})
        .with(BlocksTile{})
        .with(CombatStats{max_hp: 16, hp: 16, defense: 1 , power: 4})
        .build()
}
