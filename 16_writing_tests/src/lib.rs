pub fn add(left: usize, right: usize) -> usize {
    left + right
}

struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn can_hold(self ,other: &Rectangle)-> bool{
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(num: u32)-> u32{
    2+num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
   }

    #[test]
    fn exploration(){
        assert_ne!(4,5);
    }

    // #[test]
    // fn hit_panic(){
    //     panic!("panicking test");
    // }
#[test]
    fn can_hold_rectangles(){
        let rect1 = Rectangle{
            width: 1,
            height: 2,
        };

        let rect2 = Rectangle {
            width: 2,
            height: 4,
        };

        assert!(rect2.can_hold(&rect1));
    }

#[test]
    fn add_two_to_two(){
        assert_eq!(4, add_two(2));
    }
    
#[test]
    fn result_contains_carol(){
        let result = "it contains ca!";
        assert!(result.contains("carol"), "Result did not contain 'carol'");
    }
}
