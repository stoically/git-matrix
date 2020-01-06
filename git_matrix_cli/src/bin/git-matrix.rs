#[macro_use]
extern crate text_io;

use git_matrix::git;
use git_matrix::matrix;

#[tokio::main]
async fn main() {
    let mut config = git::get_config().unwrap();

    eprint!("Homeserver URL: ");
    let homeserver_url: String = read!("{}\n");
    eprint!("User: ");
    let user: String = read!("{}\n");
    let password = rpassword::read_password_from_tty(Some("Password: ")).unwrap();

    let client = matrix::create_client(&homeserver_url, None).unwrap();
    let session = client.log_in(user.clone(), password, None).await.unwrap();

    config
        .set_str("credential.matrix.url", &homeserver_url)
        .unwrap();
    config.set_str("credential.matrix.username", &user).unwrap();
    config
        .set_str("credential.matrix.access-token", &session.access_token)
        .unwrap();
    config
        .set_str("credential.matrix.device-id", &session.device_id)
        .unwrap();

    eprintln!("Logged in");
}
