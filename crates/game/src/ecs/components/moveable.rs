use protocol::packets::spawn_entity::EntityKind;

pub struct Moveable;

pub struct Collideable;

#[derive(Debug, Default, Copy, Clone)]
pub struct CollisionMesh {
    pub top: Option<EntityKind>,
    pub bottom: Option<EntityKind>,
    pub left: Option<EntityKind>,
    pub right: Option<EntityKind>,
}

impl CollisionMesh {
    pub fn invert(other: &CollisionMesh, other_kind: EntityKind) -> Self {
        Self {
            top: other.bottom.map(|_| other_kind),
            bottom: other.top.map(|_| other_kind),
            left: other.right.map(|_| other_kind),
            right: other.left.map(|_| other_kind),
        }
    }

    pub fn reset(&mut self) {
        self.top = None;
        self.bottom = None;
        self.left = None;
        self.right = None;
    }

    pub fn update(&mut self, other: &CollisionMesh) {
        self.top = other.top.or(self.top);
        self.bottom = other.bottom.or(self.bottom);
        self.left = other.left.or(self.left);
        self.right = other.right.or(self.right);
    }

    pub fn any(&self) -> bool {
        self.top.is_some() || self.bottom.is_some() || self.left.is_some() || self.right.is_some()
    }

    pub fn any_of_kind(&self, kind: EntityKind) -> bool {
        self.top == Some(kind)
            || self.bottom == Some(kind)
            || self.left == Some(kind)
            || self.right == Some(kind)
    }
}
