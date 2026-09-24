pub trait Run {
    type Output;
    
    async fn run(&mut self) -> Self::Output;
}
