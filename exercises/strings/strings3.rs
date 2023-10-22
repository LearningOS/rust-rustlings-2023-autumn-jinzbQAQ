// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.


fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let mut x=input.to_string();
    let mut j:u32=0;
    for i in x.chars(){
        if i==' '{
           j=j+1; 
        }
        else {
            break;
        }
    }
    while(j!=0){
        x.remove(0);
        j=j-1;
    }
    let mut m:u32=(input.len()-1).try_into().unwrap();
    while(input.chars().nth(m.try_into().unwrap()) == Some(' ') ){
            m=m-1;
    }
    while(m!=(input.len()-1).try_into().unwrap()){
        m=m+1;
        x.pop();
    }
    
    // while(x[0]==" "){
    //    x.remove(0);
    // }
    // let mut i = x.len()-1;
    // while(x[i]==" "){
      
    //     x.pop();
    //     i=x.len();
    // }
    x
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let mut x=input.to_string();
    x.push_str(" world!");
    x
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let x=input.to_string();
    x.replace("cars","balloons")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
