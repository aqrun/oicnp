use scru128;
use oicnp_api::utils::base_10_to_n;

/**
037YGCNZNYIN6ZGZQBIKT7B0B => 2QOiNhAkWh7AQFraiSI5B
037YGCNZNYIN6ZGZQBK1SP5UR => 2QOiNhAkWh7AQFrdNHMEr
037YGCNZNYIN6ZGZQBN909T5S => 2QOiNhAkWh7AQFrlp9NxS
037YGCNZNYIN6ZGZQBOPZHH7N => 2QOiNhAkWh7AQFroTWSrx
037YGCNZNYIN6ZGZQBPSED369 => 2QOiNhAkWh7AQFrrr8HT3
037YGCNZNYIN6ZGZQBT3Q3RHE => 2QOiNhAkWh7AQFrzjqVMu
037YGCNZO3ZI9QTLTZCT288L3 => 2QOiNhAqYZmnkqxe70YYf
037YGCNZO3ZI9QTLTZFIR34WZ => 2QOiNhAqYZmnkqxkyM255
037YGCNZO3ZI9QTLTZHDF9VIH => 2QOiNhAqYZmnkqxoXBlvH
037YGCNZO3ZI9QTLTZJI0AYS3 => 2QOiNhAqYZmnkqxu0ZvHl
037YGCNZO3ZI9QTLTZKNVPPJX => 2QOiNhAqYZmnkqxwMkYxf
*/
fn main() {
    for i in 0..100 {
        let id = scru128::scru128();
        let id1 = id.to_string();
        let id2 = base_10_to_n(id.to_u128() as u128, 62);
        println!("{} => {}", id1, id2);
    }
}
