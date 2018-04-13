#[cfg(test)]
mod tests {
    use std::cmp;
    struct Point {
        x:i32, y:i32 
    } 

    enum Shape {
        Point  { point : Point },  
        Line   { point0 : Point, point1 : Point },
        Circle { point : Point, radius : i32 },
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
    }
    fn min_point(v : Vec<Shape>) -> Point {
        let (head,tail) = v.split_at(1);
        let init = head[0].min_point();
        tail.iter().fold(init, | mut acc:Point, shape | {
            acc = Shape::Line { point0: acc, point1: shape.min_point() }.min_point();
            acc
        })
    }
    fn max_point(v : Vec<Shape>) -> Point {
        let (head,tail) = v.split_at(1);
        let init = head[0].max_point();
        tail.iter().fold(init, | mut acc:Point, shape | {
            acc = Shape::Line { point0: acc, point1: shape.max_point() }.max_point();
            acc
        })
    }
    fn get_shape(s : &str) -> Shape {
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
    fn point_min_point_test() {
        let p = Shape::Point { point : Point { x:-3, y:5 }};
        let c = p.min_point();
        assert_eq!(c.x, -3);
        assert_eq!(c.y, 5)
    }
    #[test]
    fn line_min_point_test() {
        let p = Shape::Line { point0 : Point { x:-3, y:5 }, point1 : Point { x:4, y:-25} };
        let c = p.min_point();
        assert_eq!(c.x, -3);
        assert_eq!(c.y, -25)
    }
    #[test]
    fn circle_min_point_test() {
        let p = Shape::Circle { point : Point { x:-3, y:5 }, radius : 10 };
        let c = p.min_point();
        assert_eq!(c.x, -13);
        assert_eq!(c.y, -5)
    }
    #[test]
    fn point_max_point_test() {
        let p = Shape::Point { point : Point { x:-3, y:5 }};
        let c = p.max_point();
        assert_eq!(c.x, -3);
        assert_eq!(c.y, 5)
    }
    #[test]
    fn line_max_point_test() {
        let p = Shape::Line { point0 : Point { x:-3, y:5 }, point1 : Point { x:4, y:-25} };
        let c = p.max_point();
        assert_eq!(c.x, 4);
        assert_eq!(c.y, 5)
    }
    #[test]
    fn circle_max_point_test() {
        let p = Shape::Circle { point : Point { x:-3, y:5 }, radius : 10 };
        let c = p.max_point();
        assert_eq!(c.x, 7);
        assert_eq!(c.y, 15)
    }
    #[test]
    fn min_point_of_a_list_of_shapes() {
        let v = vec![Shape::Point { point : Point { x:-3, y:5}}
                    ,Shape::Line { point0 : Point { x:-8, y:45 }, point1 : Point { x:4, y:-25} }
                    ,Shape::Circle { point : Point { x:-3, y:5 }, radius : 10 }];
        let c = min_point(v);
        assert_eq!(c.x,-13);
        assert_eq!(c.y,-25);

    }
    #[test]
    fn max_point_of_a_list_of_shapes() {
        let v = vec![Shape::Point { point : Point { x:-3, y:5}}
                    ,Shape::Line { point0 : Point { x:-8, y:45 }, point1 : Point { x:4, y:-25} }
                    ,Shape::Circle { point : Point { x:-3, y:5 }, radius : 10 }];
        let c = max_point(v);
        assert_eq!(c.x,7);
        assert_eq!(c.y,45);

    }
    #[test]
    fn get_shape_can_read_a_point() {
        let s = "p 3 -5";
        let sh : Shape =get_shape(s);
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
        let s = "p -34 4807";
        let sh : Shape =get_shape(s);
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
        let s = "l 3 -5 42 7";
        let sh : Shape =get_shape(s);
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
        let s = "c -2 5 42";
        let sh : Shape =get_shape(s);
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
