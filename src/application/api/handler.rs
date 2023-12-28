use tonic::{Request, Response, Status};
use crate::pbstaff;
use crate::pbstaff::{GetStaffByIdRequest, GetStaffByIdResponse};
use crate::pbstaff::staff_server_service_server::{StaffServerService};
#[derive(Default,Clone)]
pub struct StaffServiceApi{

}
impl StaffServiceApi{
    pub fn new()->Self{
        StaffServiceApi{}
    }
}

#[tonic::async_trait]
impl StaffServerService for StaffServiceApi{
    async fn get_staff_by_id(&self, request: Request<GetStaffByIdRequest>)
        -> Result<Response<GetStaffByIdResponse>, Status> {
        let req = request.get_ref();

        Ok(Response::new(GetStaffByIdResponse{
            create_at:None
        }))
    }
}