#[derive(Deserialize,Copy, Clone)]
pub struct  Address{
    id:String,
    street:String,
    state:String,
    post_code:String,
    country:String,
    staff_id:String
}