pub trait Led {
    fn red(&mut self, is_on: bool);
    fn green(&mut self, is_on: bool);
    fn blue(&mut self, is_on: bool);
}
