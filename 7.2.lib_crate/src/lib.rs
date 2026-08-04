
mod front_of_house {
    pub mod hosting;
    pub mod serving;
}

#[cfg(test)]
mod tests {
    use crate::front_of_house;


    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn front_house() {
        front_of_house::serving::take_order();
        assert_eq!(2, 2);
    }
}
