use std::collections::HashSet;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct XYZ {
    x: isize,
    y: isize,
    z: isize
}

trait Neighbors {
    fn neighbors(&self) -> Vec<XYZ>;
}

impl Neighbors for XYZ {
    fn neighbors(&self) -> Vec<XYZ> {
        vec![
            XYZ { x: self.x + 1, y: self.y, z: self.z },
            XYZ { x: self.x - 1, y: self.y, z: self.z },
            XYZ { x: self.x, y: self.y + 1, z: self.z },
            XYZ { x: self.x, y: self.y - 1, z: self.z },
            XYZ { x: self.x, y: self.y, z: self.z + 1 },
            XYZ { x: self.x, y: self.y, z: self.z - 1 },
        ]
    }
}

fn main() {
    let mut connected = HashSet::new();
    let mut border = HashSet::new();

    let middle = XYZ{x: 0, y: 0, z: 0};
    border.insert(middle);

    while !border.is_empty() && connected.len() < 10000 {
        let choice = *border.iter().next().unwrap();
        border.remove(&choice);
        connected.insert(choice);

        for neighbor in choice.neighbors() {
            if !connected.contains(&neighbor) {
                border.insert(neighbor);
            }
        }
    }
}