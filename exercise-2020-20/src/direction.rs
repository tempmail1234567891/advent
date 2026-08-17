#[derive(Debug, Hash)]
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

pub const ALL: [RelativeLocation; 4] = [
    RelativeLocation::Above,
    RelativeLocation::Below,
    RelativeLocation::OnLeft,
    RelativeLocation::OnRight,
];