//! # Pizza library
//! 
//! This library implements the workflow elements of a pizza parlor.
//! 
use std::time::{Duration, Instant};

/////////////////////////////////////////
// Resources
#[derive(Debug)]
pub struct Water;

#[derive(Debug)]
pub struct Flour;

#[derive(Debug)]
pub struct Salt;

#[derive(Debug)]
pub struct Yeast;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Dough {
    water: Water,
    flour: Flour,
    salt: Salt,
    yeast: Yeast,
}

#[derive(Debug)]
pub struct Cheese;

#[derive(Debug)]
pub struct Sauce;

#[derive(Debug)]
#[allow(dead_code)]
pub struct RawPizza {
    dough: Dough,
    cheese: Vec<Cheese>,
    sauce: Sauce,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Pizza {
    raw: RawPizza,
}

/////////////////////////////////////////
// Resource factories

/// A faucet that produces Water values with no limit. 
/// This is an infinite iterator: it never stops producing Water values.
pub struct Faucet;

impl Faucet {
    pub fn new() -> Faucet {
        Faucet
    }
}

impl Iterator for Faucet {
    type Item = Water;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: implement this function
    }
}

/// A sack of flour that produces Flour values with a limited capacity. 
/// This is a finite iterator: it stops producing Flour values after it
/// produced as many as the capacity parameter.
pub struct Sack {
    // TODO: add any field you may require
}

impl Sack {
    pub fn new(capacity: usize) -> Sack {
        // TODO: implement the constructor
    }
}

impl Iterator for Sack {
    type Item = Flour;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: implement this function
    }
}

/// A jar of yeast that produces Yeast values with a limited capacity. 
/// This is a finite iterator: it stops producing Yeast values after it
/// produced as many as the capacity parameter.
pub struct Jar;

// TODO: modify the data structure definition and provide implementation blocks
// Hint: you can use the Sack as a template (same logic here)


/// A shaker of salt that produces Salt values with a limited capacity.
/// This is a finite iterator: it stops producing Salt values after it
/// produced as many as the capacity parameter.
pub struct Shaker;

// TODO: modify the data structure definition and provide implementation blocks
// HINT: you can use the Sack as a template (same logic here) 

/// A breadboard that produces Dough values by combining Water, Flour, Salt 
/// and Yeast.
/// This is a finite iterator which encapsulates other iterators: it stops 
/// producing Dough values when any of the underlying iterator is exhausted.
/// Note: we do not hard wire the Faucet, Sack, Jar and Shaker types here, but
/// use trait bounds: this allows us to use any type that provides the necessary
/// resources.
pub struct BreadBoard<W, F, S, Y> {
    water_source: W,
    flour_source: F,
    salt_source: S,
    yeast_source: Y,
}

impl<W, F, S, Y> BreadBoard<W, F, S, Y> {
    pub fn new(
        water_source: W,
        flour_source: F,
        salt_source: S,
        yeast_source: Y,
    ) -> BreadBoard<W, F, S, Y> {
        // TODO: implement the constructor
    }
}

impl<W, F, S, Y> Iterator for BreadBoard<W, F, S, Y>
where
    W: Iterator<Item = Water>,
    F: Iterator<Item = Flour>,
    S: Iterator<Item = Salt>,
    Y: Iterator<Item = Yeast>,
{
    type Item = Dough;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: implement this function
    }
}

/// A can of sauce that produces Sauce values with a limited capacity.
/// This is a finite iterator: it stops producing Sauce values after it
/// produced as many as the capacity parameter.
pub struct Can;

// TODO: modify the data structure definition and provide implementation blocks
// HINT: you can use the Sack as a template (same logic here) 

/// A cheese grater that produces Cheese values with a limited throughput.
/// This is an infinite iterator with a twist. It produces infinite number of
/// Cheese values, but it does so at a limited rate. The throughput parameter
/// specifies how many Cheese values it can produce per second. If the caller
/// asks for more Cheese values than the throughput allows, the grater will
/// wait until it can produce the next Cheese value.
/// I.e. if the throughput is 5, the grater will produce up to 5 Cheese values
/// in any one second long time interval.
pub struct Grater {
    // TODO: add any field you may require
}

impl Grater {
    const PERIOD: Duration = Duration::from_secs(1);

    pub fn new(throughput: usize) -> Grater {
        // TODO: implement the constructor
    }
}

impl Iterator for Grater {
    type Item = Cheese;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: implement this function
        // HINT: you can rely on the std::time::Duration, std::time::Instant 
        // types, and std::thread::sleep functions here
    }
}

/// A prep table that produces RawPizza values by combining Dough, Sauce, and
/// multiple Cheese.
/// This is a finite iterator which encapsulates other iterators: it stops 
/// producing RawPizza values when any of the underlying iterator is exhausted.
/// There is a twist here: from the Cheese resource we need multiple values
/// to produce a single RawPizza. The number of Cheese values required is
/// specified by the n_cheese parameter.
/// Note: you should not hard wire the BreadBoard, Can, Grater types here, but
/// use trait bounds: this allows us to use any type that provides the necessary
/// resources.
pub struct PrepTable;

// TODO: modify the data structure definition and provide implementation blocks
// HINT: you can use the BreadBoard as a template. This has a very similar 
//       logic, except for consuming multiple Cheese values.

/// An oven that produces Pizza values by baking RawPizza.
/// This is a finite iterator which encapsulates another iterator: it stops
/// producing Pizza values when the underlying iterator is exhausted.
/// There is a twist here: baking a RawPizza takes time. The baking time is
/// specified by the baking_time parameter. I.e. when the caller asks for the
/// next Pizza value, the oven will wait for the baking_time duration before
/// producing the next Pizza value.
pub struct Oven<R> {
    // TODO: add any field you may require
}

impl<R> Oven<R> {
    pub fn new(raw_pizza_source: R, baking_time: Duration) -> Oven<R> {
        // TODO: implement the constructor
    }
}

impl<R> Iterator for Oven<R>
where
    R: Iterator<Item = RawPizza>,
{
    type Item = Pizza;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: implement this function
    }
}

#[cfg(test)]
mod tests;