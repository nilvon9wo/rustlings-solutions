struct ColorClassicStruct {
    name: String,
    hex: String,
}

struct ColorTupleStruct(String, String);

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    const GREEN: &'static str = "green";

    const GREEN_HEX: &'static str = "#00FF00";

    #[test]
    fn classic_c_structs() {
        let green = ColorClassicStruct {
            name: GREEN.to_string(),
            hex: GREEN_HEX.to_string(),
        };

        assert_eq!(green.name, GREEN);
        assert_eq!(green.hex, GREEN_HEX);
    }

    #[test]
    fn tuple_structs() {
        let green = ColorTupleStruct(GREEN.to_string(), GREEN_HEX.to_string());

        assert_eq!(green.0, GREEN);
        assert_eq!(green.1, GREEN_HEX);
    }

    #[test]
    fn unit_structs() {
        let unit_struct = UnitStruct;
        let message = format!("{:?}s are fun!", unit_struct);

        assert_eq!(message, "UnitStructs are fun!");
    }
}
