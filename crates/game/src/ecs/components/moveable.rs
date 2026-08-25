pub struct Moveable;

#[derive(Default)]
pub struct CollisionMesh {
    pub top: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}
