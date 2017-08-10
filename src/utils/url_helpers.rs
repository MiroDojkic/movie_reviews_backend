

use iron::Url;
use super::regex::{Regex, Error};

pub fn includes_path(paths: Vec<&str>, url: &Url) -> Result<bool, Error> {
    let path = url.path().join("/");
    let pattern = paths.join("|");
    let regex = Regex::new(&pattern);

    regex.map(|re| re.is_match(&path))
}
