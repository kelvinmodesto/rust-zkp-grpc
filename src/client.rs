use num_bigint::BigUint;
use std::io::stdin;
use zkp_grpc::ZKP;

use crate::zkp_auth::{
    AuthenticationAnswerRequest, AuthenticationChallengeRequest, RegisterRequest,
    auth_client::AuthClient,
};

pub mod zkp_auth {
    include!("./zkp_auth.rs");
}

#[tokio::main]
async fn main() {
    let mut buffer = String::new();
    let (alpha, beta, p, q) = ZKP::get_constants();
    let zkp = ZKP {
        alpha: alpha.clone(),
        beta: beta.clone(),
        p: p.clone(),
        q: q.clone(),
    };
    let mut client = AuthClient::connect("http://127.0.0.1:50051")
        .await
        .expect("Could not connect to the server");

    println!("Please provide username: ");
    stdin()
        .read_line(&mut buffer)
        .expect("Could not get the username from sdtin");
    let user_name = buffer.trim().to_string();
    buffer.clear();

    println!("Please provide password: ");
    stdin()
        .read_line(&mut buffer)
        .expect("Could not get the password from stdin");
    let password = BigUint::from_bytes_be(buffer.trim().as_bytes());
    buffer.clear();

    let (y1, y2) = zkp.compute_pair(&password);

    let request = RegisterRequest {
        name: user_name.clone(),
        y1: y1.to_bytes_be(),
        y2: y2.to_bytes_be(),
    };

    let _response = client
        .register(request)
        .await
        .expect("Could not register in server");

    println!("{:?}", _response);

    println!("Please provide the password to login:");
    stdin()
        .read_line(&mut buffer)
        .expect("Could not get the password from stdin");
    let password = BigUint::from_bytes_be(buffer.trim().as_bytes());
    buffer.clear();

    let k = ZKP::generate_random_lower_than(&q);
    let (r1, r2) = zkp.compute_pair(&k);

    let request = AuthenticationChallengeRequest {
        user: user_name,
        r1: r1.to_bytes_be(),
        r2: r2.to_bytes_be(),
    };

    let response = client
        .create_authentication_challenge(request)
        .await
        .expect("Could not create a authentication challenge")
        .into_inner();

    println!("{:?}", response);

    let auth_id = response.auth_id;
    let c = BigUint::from_bytes_be(&response.c);

    let s = zkp.solve(&k, &c, &password);

    let request = AuthenticationAnswerRequest {
        auth_id,
        s: s.to_bytes_be(),
    };

    let response = client
        .verify_authentication(request)
        .await
        .expect("Could not verify authentication in server")
        .into_inner();

    println!(
        "You logged in, your session_id is: {:?}",
        response.session_id
    );
}
