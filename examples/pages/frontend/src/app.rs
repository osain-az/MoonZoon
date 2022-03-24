use crate::{
    calc_page,
    header::header,
    login_page, report_page,
    router::{previous_route, router, Route},
};
use zoon::{eprintln, println, *};
use shared::{DownMsg, UpMsg};

// ------ ------
//     Types
// ------ ------

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum PageId {
    Report,
    Login,
    Calc,
    Home,
    Unknown,
}

// ------ ------
//    States
// ------ ------

#[static_ref]
pub fn logged_user() -> &'static Mutable<Option<String>> {
    Mutable::new(None)
}

#[static_ref]
fn page_id() -> &'static Mutable<PageId> {
    Mutable::new(PageId::Unknown)
}

// ------ ------
//    Helpers
// ------ ------

pub fn is_user_logged() -> bool {
    logged_user().map(Option::is_some)
}

// ------ ------
//   Commands
// ------ ------

pub fn set_page_id(new_page_id: PageId) {
    page_id().set_neq(new_page_id);
}

pub fn log_in(name: String) {
    logged_user().set(Some(name));
    router().go(previous_route().unwrap_or(Route::Root));
}

pub fn log_out() {
    logged_user().take();
    router().go(Route::Root);
}

// ------ ------
//     View
// ------ ------


pub fn root() -> impl Element {
    Column::new()
        .s(Padding::all(20))
        .s(Spacing::new(20))
        .item(header())
        .item(page())
}


fn page() -> impl Element {
    El::new().child_signal(page_id().signal().map(|page_id| match page_id {
        PageId::Report => report_page::page().into_raw_element(),
        PageId::Login => login_page::page().into_raw_element(),
        PageId::Calc => calc_page::page().into_raw_element(),
        PageId::Home => El::new().child("Welcome Home!").into_raw_element(),
        PageId::Unknown => El::new().child("404").into_raw_element(),
    }))
}


//---fOR DEBUGGING---

pub fn send_item_to_backend() {
    eprintln!("sending item ");

    Task::start(async {
        let msg = UpMsg::NewTest {
            item: "testing item".to_string(),
            // password: password().get_cloned(),
        };
       if let Err(error) = connection().send_up_msg(msg).await {
            let error = error.to_string();
            eprintln!("login request failed: {}", error);
        }
    });
}


#[static_ref]
pub fn connection() -> &'static Connection<UpMsg, DownMsg> {
    Connection::new(|down_msg, _cor_id| {
        println!("DownMsg received: {:?}", down_msg);

        match down_msg {
            DownMsg::NewItem(error) => eprintln!("authorization error: '{error}'"),
        }
    })
}
