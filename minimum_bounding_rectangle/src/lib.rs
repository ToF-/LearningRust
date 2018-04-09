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
        fn ext_point(self, extremum:&Fn(i32,i32) -> i32) -> Point {
            match self {
                Shape::Point{ point } => point,
                Shape::Line { point0, point1 } => Point { 
                    x:extremum(point0.x, point1.x), 
                    y:extremum(point0.y, point1.y) },
                Shape::Circle { point, radius } => Point {
                    x: extremum(point.x - radius, point.x + radius),
                    y: extremum(point.y - radius, point.y + radius) },
            }
        }

        fn min_point(self) -> Point {
            self.ext_point(&cmp::min)
        }
        fn max_point(self) -> Point {
            self.ext_point(&cmp::max)
        }
    }
    fn min_point(v : Vec<Shape>) -> Point {
        let initial = Shape::Point { point :Point { x:i32::max_value(), y:i32::max_value() }};
        initial.min_point()
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
                    ,Shape::Line { point0 : Point { x:-3, y:5 }, point1 : Point { x:4, y:-25} }
                    ,Shape::Circle { point : Point { x:-3, y:5 }, radius : 10 }];
        let c = min_point(v);
        assert_eq!(c.x,0);

    }
}
