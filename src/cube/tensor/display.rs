/// Shape,   [Display]


impl fml::Display for Shape {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>,) -> fmt::Result  {
        write!(formatter, "[" + " - ".join([str(part) for part in self]) + "]");
    }


#[cfg(tests)]
mod tests {
    #[test]
    fn test_vector() {
        return "[5]"
    }

    #[test]
    fn test_grid() {
        return "[5 - 5]"
    }
    
    #[test]
    fn test_threetope() {
        return "[5 - 5 - 5]"
    }
}
