use std::convert::TryFrom;

#[derive(Debug)]
pub(crate) struct Material<'a> {
    pub count: u64,
    pub name: &'a str,
}

#[derive(Debug)]
pub(crate) struct Reaction<'a> {
    pub sources: Vec<Material<'a>>,
    pub target: Material<'a>,
}

#[derive(Debug)]
pub struct InvalidFormat;

impl<'a> TryFrom<&'a str> for Material<'a> {
    type Error = InvalidFormat;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut splitted = s.split(' ');
        let count = splitted
            .next()
            .ok_or(InvalidFormat)?
            .parse()
            .map_err(|_| InvalidFormat)?;

        let name = splitted.next().ok_or(InvalidFormat)?;

        Ok(Self { count, name })
    }
}

impl<'a> TryFrom<&'a str> for Reaction<'a> {
    type Error = InvalidFormat;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut splitted = s.split(" => ");

        let sources = splitted.next().unwrap();
        let target = splitted.next().unwrap();

        let sources = sources
            .split(", ")
            .map(Material::try_from)
            .collect::<Result<Vec<Material>, _>>()?;

        let target: Material = Material::try_from(target)?;

        Ok(Self { sources, target })
    }
}
