// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / 31557600_f64 / Self::orbital_period()
    }
    fn orbital_period() -> f64;
}

macro_rules! impl_Planet {
    (for $($t:ident: $p: literal),+) => {
        $(  pub struct $t;
            impl Planet for $t {
                fn orbital_period() -> f64 {
                    $p
                }
            }
        )*
    }
}

impl_Planet!(for Earth: 1.0,
Mercury: 0.2408467,
Venus: 0.61519726,
Mars: 1.8808158,
Jupiter: 11.862615,
Saturn: 29.447498,
Uranus: 84.016846,
Neptune: 164.79132);
