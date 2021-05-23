use amethyst::core::ecs::world::Index;

#[derive(Debug)]
pub enum MoveEvent {
    PlayerHitObstacle,
    EntityMoved(EntityMoved),
    BoxPlacedOnSpot(BoxPlacedOnSpot),
}

#[derive(Debug)]
pub struct EntityMoved {
    pub id: Index,
}

#[derive(Debug)]
pub struct BoxPlacedOnSpot {
    pub is_correct_spot: bool,
}

