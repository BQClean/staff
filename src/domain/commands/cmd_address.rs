#[derive(Deserialize,Serialize,Clone)]
pub struct  Address{
   pub id:String,
   pub street:String,
   pub  state:String,
   pub post_code:String,
   pub country:String,
   pub staff_id:String
}