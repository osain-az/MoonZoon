use moon::*;
use shared::UpMsg;
async fn frontend() -> Frontend {
    Frontend::new().title("Pages  example").append_to_head(
        "
        <style>
            html {
                background-color: black;
                color: lightgray;
            }
        </style>",
    )
}

pub async fn up_msg_handler(req: UpMsgRequest<UpMsg>) {
  eprintln!(" Msg request {:?}", req.up_msg);
}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_| {}).await
}
