
fn main() {
    use std::io;
    use std::cmp;
    struct Point {
        x:i32, y:i32 
    } 

    enum Shape {
        Point  { point:Point },  
        Line   { point0:Point, point1:Point },
        Circle { point:Point, radius:i32 },
    }

    impl Shape {
        fn ext_point(&self, extremum:&Fn(i32,i32) -> i32) -> Point {
            match *self {
                Shape::Point{ ref point } => Point { x:point.x, y:point.y },
                Shape::Line { ref point0, ref point1 } => Point { 
                    x:extremum(point0.x, point1.x), 
                    y:extremum(point0.y, point1.y) },
                Shape::Circle { ref point, ref radius } => Point {
                    x: extremum(point.x - radius, point.x + radius),
                    y: extremum(point.y - radius, point.y + radius) },
            }
        }

        fn min_point(&self) -> Point {
            self.ext_point(&cmp::min)
        }
        fn max_point(&self) -> Point {
            self.ext_point(&cmp::max)
        }
        fn minimum_bounding_rectangle(&self) -> Shape {
            Shape::Line { point0:self.min_point(), point1:self.max_point() }
        }
        fn minimum_bounding_rectangle_with(&self, other:&Shape) -> Shape {
            if let Shape::Line { point0:p0, point1:p1 } = self.minimum_bounding_rectangle() {
                if let Shape::Line { point0:p2, point1:p3 } = other.minimum_bounding_rectangle() {
                    let p4 = Shape::Line { point0:p0, point1:p2}.min_point();
                    let p5 = Shape::Line { point0:p1, point1:p3}.max_point();
                    return Shape::Line { point0:p4, point1:p5 }
                }
            }
            panic!("unexpected condition")
        }
    }
    fn minimum_bounding_rectangle(v:Vec<Shape>) -> Shape {
        let (head,tail) = v.split_at(1);
        let init = head[0].minimum_bounding_rectangle();
        tail.iter().fold(init, | mut acc, shape | {
            acc = acc.minimum_bounding_rectangle_with(shape);
            acc
        })
    }
    fn get_shape(s:&str) -> Shape {
        let inputs:Vec<&str> = s.trim().split(" ").collect();
        match inputs[0] {
            "p" => { 
                let x:i32 = inputs[1].parse().expect("Not an integer!");
                let y:i32 = inputs[2].parse().expect("Not an integer!");
                Shape::Point { point:Point { x:x, y:y }}
                },
            "l" => {
                let x0:i32 = inputs[1].parse().expect("Not an integer!");
                let y0:i32 = inputs[2].parse().expect("Not an integer!");
                let x1:i32 = inputs[3].parse().expect("Not an integer!");
                let y1:i32 = inputs[4].parse().expect("Not an integer!");
                Shape::Line { point0:Point { x:x0, y:y0 }, point1:Point {x:x1, y:y1 }}
            },
            "c" => {
                let x:i32 = inputs[1].parse().expect("Not an integer!");
                let y:i32 = inputs[2].parse().expect("Not an integer!");
                let r:i32 = inputs[3].parse().expect("Not an integer!");
                Shape::Circle { point:Point { x:x, y:y }, radius:r }
            }
             _ => panic!("unknown shape description")
        }
    }
    fn read_input() -> io::Result<String> {
        let mut input = String::new();

        io::stdin().read_line(&mut input)?;


        Ok(input)
    }

    let max_cases = read_input().expect("no input").trim().parse().expect("not a number");
    for _ in 0..max_cases {
        let max_shapes = read_input().expect("no input").trim().parse().expect("not a number");
        let mut shapes:Vec<Shape> = vec![]; 
        for _ in 0..max_shapes {
            match read_input() {
                Ok(line) => { 
                    shapes.push(get_shape(&line))
                }, 
                Err(_) => {
                    break
                }
            }
        }
        if let Shape::Line { point0:p0, point1:p1 } = minimum_bounding_rectangle(shapes) {
            println!("{} {} {} {}", p0.x, p0.y, p1.y, p1.y)
        }else {
            panic!("unexpected condition")
        }
        read_input();
    }
}
