#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Default,
    XFlip,
    YFlip,
    XYFlip,
}

#[derive(Debug)]
pub enum RelativeLocation {
    Above,
    Below,
    OnLeft,
    OnRight,
}
