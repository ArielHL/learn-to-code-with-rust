#[derive(Debug)]
struct Location {
    name: String,
    treasure: u32,
}

struct Map<'a> {
    locations: &'a [Location],
}

impl<'a> Map<'a> {
    fn explore<F>(&self, mut action: F)
    where
        F: FnMut(&Location),
    {
        for location in self.locations {
            action(location);
        }
    }
}
fn main() {
    let locations = [
        Location {
            name: "Location 1".to_string(),
            treasure: 100,
        },
        Location {
            name: "Location 2".to_string(),
            treasure: 200,
        },
        Location {
            name: "Location 3".to_string(),
            treasure: 300,
        },
    ];
    let map = Map {
        locations: &locations,
    };

    let mut total_treasures = 0;
    let action_closure = |location: &Location| total_treasures += location.treasure;
    map.explore(action_closure);

    println!("Total treasures: {}", total_treasures);

    let mut location_names: Vec<String> = Vec::new();
    let action_strings = |location: &Location| {
        location_names.push(location.name.clone());
    };
    map.explore(action_strings);
    println!("Location names: {:?}", location_names);
}
