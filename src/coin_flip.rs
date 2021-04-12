use crate::coin_flip::FlipResult::{Both, Heads, Tails};
use core::fmt;
use rand::distributions::Bernoulli;
use rand::prelude::{Distribution, ThreadRng};
use wasm_bindgen::prelude::*;

pub struct Coin;

impl Coin {
    pub fn flip(
        mut rng: &mut ThreadRng,
        thumb_count: usize,
        times: usize,
    ) -> impl Iterator<Item = FlipResult> + '_ {
        std::iter::from_fn(move || {
            let coins_per_flip = 2_usize.saturating_pow(thumb_count as u32);
            let (mut seen_heads, mut seen_tails) = (false, false);
            for val in Bernoulli::new(0.5)
                .unwrap()
                .sample_iter(&mut rng)
                .take(coins_per_flip)
            {
                if val {
                    seen_heads = true;
                } else {
                    seen_tails = true;
                }
                if seen_heads && seen_tails {
                    break;
                }
            }
            if seen_heads && seen_tails {
                Some(Both)
            } else if seen_heads {
                Some(Heads)
            } else {
                Some(Tails)
            }
        })
        .take(times)
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum FlipResult {
    Heads,
    Tails,
    Both,
}

impl fmt::Display for FlipResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub enum FlipStrategy {
    Conservative,
    Greedy,
}

pub struct GameContext {}
