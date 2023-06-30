#[derive(Clone)]
pub enum CellSignal {
    LeftClick,
    RightClick,
    NeighborReveal,
    NeighborFlagSet,
    NeighborFlagUnset,
    Stop,
}
