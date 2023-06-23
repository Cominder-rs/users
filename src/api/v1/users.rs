use tonic::{Request, Response, Status};
use users_proto::{*, users_v1_server::UsersV1};

pub struct UsersApiV1;

#[tonic::async_trait]
impl UsersV1 for UsersApiV1{
    async fn get_user_by_id(&self, req: Request<UserId>) -> Result<Response<User>, Status>{
        Err(Status::unimplemented(""))
    }

    async fn test_size(&self, req: Request<()>) -> Result<Response<TestUserWeight>, Status> {
        Ok(Response::new(TestUserWeight {
            firstname: "Anton".into(),
            lastname: "Lisovskiy".into(),
            city: "Tyumen".into(),
            about: " Way nor furnished sir procuring therefore but. Warmth far manner myself active are cannot called. Set her half end girl rich met. Me allowance departure an curiosity ye. In no talking address excited it conduct. Husbands debating replying overcame blessing he it me to domestic. ".into(),
            id: 1,
        }))
    }
}