// `ty` matcher accepts trait object types

macro_rules! m {
    ($t: ty) => ( let _: $t; )
}

fn main() {
    m!(dyn Copy + Send + 'static);
    //~^ ERROR the trait `Copy` is not dyn compatible
    m!(dyn 'static + Send);
    m!(dyn 'static +); //~ ERROR at least one trait is required for an object type
}
