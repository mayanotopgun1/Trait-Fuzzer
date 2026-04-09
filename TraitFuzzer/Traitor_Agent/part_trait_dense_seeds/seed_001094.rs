struct LoadedObject {
    bodies: Vec<Body>,
    color: Color,
}

struct Body;

#[derive(Clone)]
struct Color;

struct Graphic {
    color: Color,
}

trait Convertible {
    fn convert(self) -> (Vec<Body>, Vec<Graphic>);
}

impl Convertible for Vec<LoadedObject> {
    fn convert(self) -> (Vec<Body>, Vec<Graphic>) {
        self.into_iter()
            .flat_map(|LoadedObject { bodies, color, .. }| {
                bodies.into_iter().map(move |body| {
                    (
                        body,
                        Graphic {
                            color: color.clone(),
                        },
                    )
                })
            })
            .unzip()
    }
}

fn main() {
    let objects = vec![/* some LoadedObjects */];
    let (bodies, graphics) = objects.convert();
}