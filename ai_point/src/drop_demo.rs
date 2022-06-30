pub struct CustomSmartPointer{
    pub data :String,
}
impl Drop for CustomSmartPointer{
    fn drop(&mut self) {
        println!("exec drop CustomSmartPoint for data : {}",self.data);
    }
}