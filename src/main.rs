mod user;
use user::User;

fn main(){
    // In an array i created 3 different user x[0] = female, x[1] = male and x[2] = LGBT.
    let x = [User::new(1), User::new(2), User::new(3)];
    // and i run the speak_about_yourself function for every single person in the array using for.
    for i in x {
        i.speak_about_yourself();
    }
}
// Then its end!