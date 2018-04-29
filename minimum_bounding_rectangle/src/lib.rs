#[cfg(test)]
mod tests {
    use std::cmp;
    use spectral::prelude.*
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
    
    #[test]
    fn point_minimum_bounding_rectangle() {
        let p = Shape::Point { point:Point { x:-3, y:5 }}.minimum_bounding_rectangle();
        if let Shape::Line { point0, point1 } = p {
            assert_eq!(point0.x, -3); assert_eq!(point0.y,  5); assert_eq!(point1.x, -3); assert_eq!(point1.y,  5);
        }
    }
    #[test]
    fn line_point_minimum_bounding_rectangle() {
        let l = Shape::Line { point0:Point { x:4, y:-25 }, point1:Point { x:-3, y:5}}.minimum_bounding_rectangle();
        if let Shape::Line { point0, point1 } = l {
            assert_eq!(point0.x, -3); assert_eq!(point0.y,-25); assert_eq!(point1.x,  4); assert_eq!(point1.y,  5);
        }
    }
    #[test]
    fn circle_point_minimum_bounding_rectangle() {
        let c = Shape::Circle { point:Point { x:-3, y:5 }, radius:10 }.minimum_bounding_rectangle();
        if let Shape::Line { point0, point1 } = c {
            assert_eq!(point0.x,-13); assert_eq!(point0.y, -5); assert_eq!(point1.x,  7); assert_eq!(point1.y, 15);
        }
    }
    #[test]
    fn minimum_bounding_rectangle_with_other_shape() {
        let c = Shape::Circle { point:Point { x:-3, y:5 }, radius:10 };
        let l = Shape::Line { point0:Point { x:4, y:-25 }, point1:Point { x:-3, y:5}};
        if let Shape::Line { point0, point1 } = c.minimum_bounding_rectangle_with(&l) {
            assert_eq!(point0.x,-13); assert_eq!(point0.y,-25); assert_eq!(point1.x,  7); assert_eq!(point1.y, 15);
        }
    }
    #[test]
    fn minimum_bounding_rectangle_a_list_of_shapes() {
        if let Shape::Line { point0, point1 } = minimum_bounding_rectangle(vec![Shape::Point { point:Point { x:-3, y:5}}
                    ,Shape::Line { point0:Point { x:-8, y:45 }, point1:Point { x:4, y:-25} }
                    ,Shape::Circle { point:Point { x:-3, y:5 }, radius:10 }]) {
            assert_eq!(point0.x,-13); assert_eq!(point0.y,-25); assert_eq!(point1.x,  7); assert_eq!(point1.y, 45);
        }
    }
    #[test]
    fn get_shape_can_read_a_point() {
        let sh:Shape =get_shape("p 3 -5");
        match sh {
            Shape::Point { point } => { 
               assert_eq!(point.x, 3);
               assert_eq!(point.y, -5);
            }
            _ => {
                assert!(false)
            }
        }
    }
    #[test]
    fn get_shape_can_read_any_point() {
        let sh:Shape =get_shape("p -34 4807");
        match sh {
            Shape::Point { point } => { 
               assert_eq!(point.x, -34);
               assert_eq!(point.y, 4807);
            }
            _ => {
                assert!(false)
            }
        }
    }
    #[test]
    fn get_shape_can_read_a_line() {
        let sh:Shape =get_shape("l 3 -5 42 7");
        match sh {
            Shape::Line { point0, point1 } => { 
               assert_eq!(point0.x, 3);
               assert_eq!(point0.y, -5);
               assert_eq!(point1.x, 42);
               assert_eq!(point1.y, 7);
            }
            _ => {
                assert!(false)
            }
        }
    }
    #[test]
    fn get_shape_can_read_a_circle() {
        let sh:Shape =get_shape("c -2 5 42");
        match sh {
            Shape::Circle { point, radius } => { 
               assert_eq!(point.x, -2);
               assert_eq!(point.y, 5);
               assert_eq!(radius, 42);
            }
            _ => {
                assert!(false)
            }
        }
    }

}
