use std::borrow::Borrow;

#[derive(Clone, PartialEq)]
pub struct ChatMsg {
    id: Option<i32>,
    pub message: String,
    pub user_type: String,
}

impl ChatMsg {
    pub fn new(id: i32, message: String, user_type: String) -> Self {
        ChatMsg {
            id: Some(id),
            message: message,
            user_type: user_type,
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id.to_owned().expect("Id is null")
    }
}

#[derive(Clone)]
pub struct ChatCtx {
    pub chat_msgs: Vec<ChatMsg>,
    pub curr_idx: i32,
}

impl ChatCtx {
    pub fn new() -> Self {
        ChatCtx { chat_msgs: Vec::<ChatMsg>::new(), curr_idx: 0 }
    }

    pub fn add_msg(&mut self, msg: String, user_type: String) -> i32 {
        self.chat_msgs.push(ChatMsg {
            id: Some(self.curr_idx),
            message: msg,
            user_type: user_type,
        });

        self.curr_idx = self.curr_idx + 1;
        self.curr_idx
    }
}
