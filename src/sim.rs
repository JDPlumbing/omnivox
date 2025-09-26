use chronovox::{Timeline, ChronoEvent, EventKind, UvoxId, TimeDelta};
use objex::{Objex, Shape, MaterialLink};
use uuid::Uuid;
use std::collections::HashMap;

pub struct SimWorld {
    pub objects: HashMap<UvoxId, Objex>,
    pub timeline: Timeline,
}

impl SimWorld {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            timeline: Timeline::new(),
        }
    }

    pub fn bootstrap_world() -> Self {
        let mut world = Self::new();

        // Earth’s canonical ID
        let earth_id = UvoxId::earth(0, 0, 0);

        // Build a Shape explicitly instead of json! macro
        let earth_shape = Shape {
            geometry: serde_json::json!({
                "type": "sphere",
                "radius": 6371.0
            }),
        };

        // Earth Objex
        let earth = Objex {
            entity_id: Uuid::nil(),
            name: "Earth".into(),
            shape: earth_shape,
            material: MaterialLink {
                category_id: Uuid::parse_str("cccccccc-cccc-cccc-cccc-cccccccc0004").unwrap(),
                properties: serde_json::json!({ "composition": "rock" }),
            },
        };

        world.objects.insert(earth_id.clone(), earth);

        // Add spawn event at tick 0
        let event = ChronoEvent {
            id: earth_id,
            t: TimeDelta::from_ticks(0, "nanoseconds"),
            kind: EventKind::Spawn,
            payload: None,
        };
        world.timeline.push(event);

        world
    }
}
