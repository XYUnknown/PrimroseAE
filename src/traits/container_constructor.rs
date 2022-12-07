pub trait ContainerConstructor {
    type Impl: ?Sized;
    type Bound: ?Sized;
    fn new() -> Box<Self::Bound>;
}