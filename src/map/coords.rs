use std::ops::Sub;

use bevy::prelude::*;
use rand::{random, seq::SliceRandom, thread_rng};

pub const HEX_CIRCUMRADIUS: f32 = 60.0;
pub const MARGIN: f32 = HEX_CIRCUMRADIUS * 0.1;

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexCoord {
  pub q: i32,
  pub r: i32,
}

impl HexCoord {
  pub fn new(q: i32, r: i32) -> Self {
    Self { q, r }
  }

  pub fn to_cube(self) -> (i32, i32, i32) {
    let x = self.q;
    let z = self.r;
    let y = -x - z;
    (x, y, z)
  }

  pub fn from_cube(x: i32, y: i32, z: i32) -> Self {
    assert_eq!(x + y + z, 0, "Invalid cube coordinates");
    Self { q: x, r: z }
  }

  pub fn neighbors(self) -> [HexCoord; 6] {
    const DIRECTIONS: [(i32, i32); 6] = [
      (1, 0), (1, -1), (0, -1),
      (-1, 0), (-1, 1), (0, 1),
    ];
    DIRECTIONS.map(|(dq, dr)| HexCoord::new(self.q + dq, self.r + dr))
  }

  pub fn within_distance(self, distance: u32) -> Vec<HexCoord> {
    let signed_distance = distance as i32;
    let mut tiles = Vec::new();
    for dq in -signed_distance..=signed_distance {
      for dr in (-signed_distance).max(-dq - signed_distance)..=(signed_distance).min(-dq + signed_distance) {
        let q = self.q + dq;
        let r = self.r + dr;
        tiles.push(HexCoord::new(q, r));
      }
    }
    tiles
  }

  pub fn modulus(self) -> u32 {
    let (x, y, z) = self.to_cube();
    ((x.abs() + y.abs() + z.abs()) / 2).try_into().unwrap()
  }

  pub fn distance(self, other: HexCoord) -> u32 {
    (self - other).modulus()
  }

  pub fn to_cartesian(self) -> Vec2 {
    let x = 3.0_f32.sqrt() * self.q as f32 + (3.0_f32.sqrt() / 2.0) * self.r as f32;
    let y = (3.0 / 2.0) * self.r as f32;
    Vec2 { x, y } * HEX_CIRCUMRADIUS
  }

  pub fn random_in_distance(distance: u32) -> Self {
    let mut q = (random::<u32>() % distance + 1) as i32;
    let mut r = distance as i32 - q;
    if random::<bool>() {
      q *= -1;
      r *= -1;
    }
    let (x, y, z) = HexCoord::new(q, r).to_cube();
    let mut coords = vec![x, y, z];
    coords.shuffle(&mut thread_rng());
    HexCoord::from_cube(coords[0], coords[1], coords[2])
  }

  pub fn check_modifier(self) -> i32 {
    (self.modulus() / 2) as i32
  }
}

impl Sub for HexCoord {
  type Output = HexCoord;

  fn sub(self, rhs: Self) -> Self::Output {
    Self {
      q: self.q - rhs.q,
      r: self.r - rhs.r,
    }
  }
}
