use crate::prelude::*;
use legion::systems::CommandBuffer;
use ron::de::from_reader;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs::File;

// This struct is reflected in the template.ron
// Deserialize trait lets serde know we want to deserialize this truct
// all containing stuff must also support this trait
// MOST Rust built-in types support this directive
#[derive(Clone, Deserialize, Debug)]
pub struct Template {
    pub entity_type: EntityType,
    pub name: String,
    pub glyph: char,
    pub levels: HashSet<usize>,
    pub frequency: i32,
    pub provides: Option<Vec<(String, i32)>>,
    pub hp: Option<i32>,
    pub base_damage: Option<i32>,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub enum EntityType {
    Enemy,
    Item,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Templates {
    pub entities: Vec<Template>,
}

// Notice that Templates is Vec of Template and check the load() function
// from_reader is from RON and translates using serde somehow into the Templates struct, very nice
impl Templates {
    pub fn load() -> Self {
        let file =
            File::open("resources/template.ron").expect("Failed opening template.ron file!!!");
        from_reader(file).expect("Unable to load templates")
    }

    pub fn spawn_entities(
        &self,
        ecs: &mut World,
        rng: &mut RandomNumberGenerator,
        level: usize,
        spawn_points: &[Point],
        guaranteed_monster_spawn_points: &[Point],
    ) {
        let mut available_entites = Vec::new();
        // Fill our array of available_entites
        self.entities
            .iter()
            // Make sure entity is on this level
            .filter(|e| e.levels.contains(&level))
            .for_each(|t| {
                for _ in 0..t.frequency {
                    available_entites.push(t);
                }
            });

        // Grab available monsters
        let mut available_monsters = Vec::new();
        self.entities
            .iter()
            .filter(|e| e.levels.contains(&level) && e.entity_type == EntityType::Enemy)
            .for_each(|t| {
                for _ in 0..t.frequency {
                    available_monsters.push(t);
                }
            });

        // Create a command buffer with spawn commands for each spawn point
        let mut command_buffer = CommandBuffer::new(ecs);
        spawn_points.iter().for_each(|point| {
            if let Some(entity) = rng.random_slice_entry(&available_entites) {
                self.spawn_entity(point, entity, &mut command_buffer);
            }
        });
        guaranteed_monster_spawn_points.iter().for_each(|point| {
            if let Some(monster_entity) = rng.random_slice_entry(&available_monsters) {
                self.spawn_entity(point, monster_entity, &mut command_buffer);
            }
        });
        command_buffer.flush(ecs);
    }

    fn spawn_entity(&self, pt: &Point, template: &Template, commands: &mut CommandBuffer) {
        // Entities share a base-set of components that make them viable in the world
        // add those then match type to add specifics
        let entity = commands.push((
            pt.clone(),
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437(template.glyph),
            },
            Name(template.name.clone()),
        ));

        // Now match type
        match template.entity_type {
            EntityType::Item => commands.add_component(entity, Item {}),
            EntityType::Enemy => {
                commands.add_component(entity, Enemy {});
                commands.add_component(entity, FieldOfView::new(6));
                commands.add_component(entity, ChasingPlayer);
                commands.add_component(
                    entity,
                    Health {
                        current: template.hp.unwrap(),
                        max: template.hp.unwrap(),
                    },
                );
            }
        }

        // Item provides? Add correct component
        if let Some(effects) = &template.provides {
            effects
                .iter()
                .for_each(|(provides, n)| match provides.as_str() {
                    "Healing" => commands.add_component(entity, ProvidesHealing { amount: *n }),
                    "MagicMap" => commands.add_component(entity, ProvidesDungeonMap {}),
                    "GroundStomp" => commands.add_component(entity, ProvidesStun {}),
                    _ => {
                        println!("we don't know how to provide {}", provides)
                    }
                });
        }

        if let Some(damage) = &template.base_damage {
            commands.add_component(entity, Damage(*damage));
            if template.entity_type == EntityType::Item {
                commands.add_component(entity, Weapon {});
            }
        }
    }
}
