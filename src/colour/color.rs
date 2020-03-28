mod colour

use std::str::FromStr;

impl FromStr {
type Err = ();

    fn from_str(src: &str) -> Result<Self, SElf::Err> {
        let src = src.to_lowercase();

        match src.as_ref() {
            "red" => Ok("31")
        }
    }
}