// This function returns how much ice cream there is left in the fridge.
// If it's before 22:00 (24-hour system), then 5 scoops are left. At 22:00,
// someone eats it all, so no ice cream is left (value 0). Return `None` if
// `hour_of_day` is higher than 23.

use chrono::prelude::*;

fn maybe_ice_cream(hour_of_day: u16) -> Option<u16> {
    // TODO: Complete the function body.
     let utc: DateTime<Utc> = Utc::now(); 

    let option = Option::Some(hour_of_day);

    if hour_of_day < 22{
        let ice = Some(5);
        ice

    }else if hour_of_day <= 23 {
        let ice = Some(0);
        ice
    }else {
        return  None
        
    }




}

fn main() {
    // You can optionally experiment here.
    let utc: DateTime<Utc> = Utc::now(); 
  println!( " time {:?}", utc.time().hour());

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get the value contained in the
        // Option?
        let ice_creams = maybe_ice_cream(12);
        let ice_creams = ice_creams.unwrap();

        assert_eq!(ice_creams, 5); // Don't change this line.
    }

    #[test]
    fn check_ice_cream() {
        assert_eq!(maybe_ice_cream(0), Some(5));
        assert_eq!(maybe_ice_cream(9), Some(5));
        assert_eq!(maybe_ice_cream(18), Some(5));
        assert_eq!(maybe_ice_cream(22), Some(0));
        assert_eq!(maybe_ice_cream(23), Some(0));
        assert_eq!(maybe_ice_cream(24), None);
        assert_eq!(maybe_ice_cream(25), None);
    }
}
