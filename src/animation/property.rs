pub enum Property {

}

pub enum PathProperty {

}

pub enum BaseProperty {
    Property(Property),
    PathProperty(PathProperty)
}