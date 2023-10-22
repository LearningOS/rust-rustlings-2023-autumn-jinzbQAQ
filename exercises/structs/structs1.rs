// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.

#[warn(dead_code)]
struct ColorClassicStruct {
    red:i32,
    green:i32,
    blue:i32,
}
#[derive(Debug)]
struct ColorTupleStruct(i32,i32,i32);


#[derive(Debug)]
struct UnitLikeStruct(String);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let green =ColorClassicStruct{
            red : 0,
            green : 255,
            blue : 0

        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let green =ColorTupleStruct(0,255,0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit-like struct!
        let unit_like_struct=UnitLikeStruct("UnitLikeStructs are fun!".to_string());
        let message = format!("{:?}", &unit_like_struct.0);

        assert_eq!(message, "\"UnitLikeStructs are fun!\"");
    }
}
fn main(){

}