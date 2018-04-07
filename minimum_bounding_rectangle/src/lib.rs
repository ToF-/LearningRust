#[cfg(test)]
mod tests {
    use std::cmp;

    struct Coords {
        x:i32, y:i32 
    }
    enum Shape {
        Point  { coords : Coords },
        Line   { coords0 : Coords, coords1 : Coords },
        Circle { coords : Coords, radius : i32 },
    }

    impl Shape {
        fn min_coords(self) -> Coords {
            match self {
                Shape::Point { coords } => coords, 
                Shape::Line { coords0, coords1 } => Coords { 
                    x:cmp::min(coords0.x, coords1.x), 
                    y:cmp::min(coords0.y, coords1.y) },
                Shape::Circle { coords, radius } => Coords {
                    x: coords.x - radius,
                    y: coords.y - radius },
            }
        }
        fn max_coords(self) -> Coords {
            match self {
                Shape::Point { coords } => coords, 
                Shape::Line { coords0, coords1 } => Coords { 
                    x:cmp::max(coords0.x, coords1.x), 
                    y:cmp::max(coords0.y, coords1.y) },
                Shape::Circle { coords, radius } => Coords {
                    x: coords.x + radius,
                    y: coords.y + radius },
            }
        }
    }
    #[test]
    fn point_min_coords_test() {
        let p = Shape::Point { coords : Coords { x:-3, y:5 }};
        let c = p.min_coords();
        assert_eq!(c.x, -3);
        assert_eq!(c.y, 5)
    }
    #[test]
    fn line_min_coords_test() {
        let p = Shape::Line { coords0 : Coords { x:-3, y:5 }, coords1 : Coords { x:4, y:-25} };
        let c = p.min_coords();
        assert_eq!(c.x, -3);
        assert_eq!(c.y, -25)
    }
    #[test]
    fn circle_min_coords_test() {
        let p = Shape::Circle { coords : Coords { x:-3, y:5 }, radius : 10 };
        let c = p.min_coords();
        assert_eq!(c.x, -13);
        assert_eq!(c.y, -5)
    }
    #[test]
    fn point_max_coords_test() {
        let p = Shape::Point { coords : Coords { x:-3, y:5 }};
        let c = p.max_coords();
        assert_eq!(c.x, -3);
        assert_eq!(c.y, 5)
    }
    #[test]
    fn line_max_coords_test() {
        let p = Shape::Line { coords0 : Coords { x:-3, y:5 }, coords1 : Coords { x:4, y:-25} };
        let c = p.max_coords();
        assert_eq!(c.x, 4);
        assert_eq!(c.y, 5)
    }
    #[test]
    fn circle_max_coords_test() {
        let p = Shape::Circle { coords : Coords { x:-3, y:5 }, radius : 10 };
        let c = p.max_coords();
        assert_eq!(c.x, 7);
        assert_eq!(c.y, 15)
    }
}
