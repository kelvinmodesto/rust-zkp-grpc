use num_bigint::BigUint;
use rust_zkp_grpc::ZKP;
use std::collections::HashMap;
use std::sync::Mutex;
use tonic::{Code, Request, Response, Status, transport::Server};

pub mod zkp_auth {
    include!("./zkp_auth.rs");
}

use zkp_auth::{
    AuthenticationAnswerRequest, AuthenticationAnswerResponse, AuthenticationChallengeRequest,
    AuthenticationChallengeResponse, RegisterRequest, RegisterResponse,
    auth_server::{Auth, AuthServer},
};

#[derive(Debug, Default)]
struct AuthImpl {
    pub user_info: Mutex<HashMap<String, UserInfo>>,
    pub auth_id_to_user: Mutex<HashMap<String, String>>,
}

#[derive(Debug, Default)]
pub struct UserInfo {
    // registration
    pub user_name: String,
    pub y1: BigUint,
    pub y2: BigUint,
    // authorization
    pub r1: BigUint,
    pub r2: BigUint,
    //verification
    pub c: BigUint,
    pub s: BigUint,
    pub session_id: String,
}

#[tonic::async_trait]
impl Auth for AuthImpl {
    async fn register(
        &self,
        req: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        println!("Processing Register: {:?}", req);
        let request = req.into_inner();
        let user_name = request.name;

        let mut user_info = UserInfo::default();

        user_info.user_name = user_name.clone();
        user_info.y1 = BigUint::from_bytes_be(&request.y1);
        user_info.y2 = BigUint::from_bytes_be(&request.y2);

        let mut user_info_hashmap = &mut self.user_info.lock().unwrap();
        user_info_hashmap.insert(user_name, user_info);

        Ok(Response::new(RegisterResponse {}))
    }
    async fn create_authentication_challenge(
        &self,
        req: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        println!("Processing Register: {:?}", req);
        let request = req.into_inner();
        let user_name = request.user;

        let mut user_info_hashmap = &mut self.user_info.lock().unwrap();
        if let Some(user_info) = user_info_hashmap.get_mut(&user_name) {
            let (_, _, _, q) = ZKP::get_constants();
            let c = ZKP::generate_random_lower_than(&q);
            let auth_id = "sdkjfsf".to_string();

            user_info.c = c.clone();
            user_info.r1 = BigUint::from_bytes_be(&request.r1);
            user_info.r2 = BigUint::from_bytes_be(&request.r2);

            let auth_id_to_user = &mut self.auth_id_to_user.lock().unwrap();
            auth_id_to_user.insert(auth_id.clone(), user_name.clone());

            Ok(Response::new(AuthenticationChallengeResponse {
                auth_id,
                c: c.to_bytes_be(),
            }))
        } else {
            Err(Status::new(
                Code::NotFound,
                format!("User: {} not found in database", user_name),
            ))
        }
    }

    async fn verify_authentication(
        &self,
        req: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() {
    let address = "127.0.0.1:50051".to_string();
    println!("Running the server in {}", address);

    let auth_impl = AuthImpl::default();

    Server::builder()
        .add_service(AuthServer::new(auth_impl))
        .serve(address.parse().expect("Could not convert the address"))
        .await
        .unwrap();
}
