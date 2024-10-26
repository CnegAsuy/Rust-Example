
// I try to work with OOP in rust actually. There is the my code :D.
use self::Gender::*;
// I created the Gender Enum which hold the gender value types
enum Gender {
    MALE,
    FEMALE,
    LGBT
}

// Then this is the User Struct i just put 3 value  because i don't need a lot. And as you can see I use gender as Gender enum.
pub struct User {
    name: String,
    age: u8,
    gender: Gender,


}

// This is the section which contains the functions about User i created 2 functions.
impl User {
    // This code is in the development so my new func is so basit it takes a parameter if this parameter equal to 1 it creates a female
    // if 2 it creates a male and if another it creates a LGBT gender. I used thisfor debug and try. 
    pub fn new(x: u32) -> User {
        if x == 1 {
            User {
                gender: FEMALE,
                name: String::from("bala"),
                age: 12,
            }
        } else if x == 2 {
            User {
                gender: MALE,
                name: String::from("mete"),
                age: 12,
            }
        } else {
            User {
                gender: LGBT,
                name: String::from("deniz"),
                age: 12,
            }
        }
    }

    // This is the second function, its about introduction itself.
    pub fn speak_about_yourself(&self) {
        let gender_sentence: String;
        // I created a gender_sentence and i changed its value for it Gender, using match.
        match self.gender {
            MALE => gender_sentence = String::from("I'm male"),
            FEMALE => gender_sentence = String::from("I'm female"),
            LGBT => gender_sentence = String::from("I support LGBT!")
        }
        // And printed its name, age and it gender_sentence.
        println!("Hello my name is {}, I'm {} years old and {}", self.name, self.age, gender_sentence);
    }

}