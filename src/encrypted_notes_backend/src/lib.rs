use crate::notes::*;
use ic_cdk::api::caller as caller_api;
use ic_cdk::export::candid::{candid_method, export_service};
use ic_cdk::export::Principal;
use ic_cdk_macros::*;
use std::cell::RefCell;

mod notes;

thread_local! {
    static NOTES: RefCell<Notes> = RefCell::default();
}

// 関数をコールしたユーザーのプリンシパルを取得します。
fn caller() -> Principal {
    let caller = caller_api();

    // 匿名のプリンシパルを禁止します(ICキャニスターの推奨されるデフォルトの動作)。
    if caller == Principal::anonymous() {
        panic!("Anonymous principal is not allowed");
    }
    caller
}

#[query(name = "getNotes")]
#[candid_method(query)]
fn get_notes() -> Vec<EncryptedNote> {
    let caller = caller();
    NOTES.with(|notes| notes.borrow().get_notes(caller))
}

#[update(name = "addNote")]
#[candid_method(update)]
fn add_note(encrypted_text: String) {
    let caller = caller();
    NOTES.with(|notes| {
        notes.borrow_mut().add_note(caller, encrypted_text);
    })
}

#[update(name = "deleteNote")]
#[candid_method(update)]
fn delete_note(id: u128) {
    let caller = caller();
    NOTES.with(|notes| {
        notes.borrow_mut().delete_note(caller, id);
    })
}

#[update(name = "updateNote")]
#[candid_method(update)]
fn update_note(new_note: EncryptedNote) {
    let caller = caller();
    NOTES.with(|notes| {
        notes.borrow_mut().update_note(caller, new_note);
    })
}

// The workaround to generate did files automatically
fn export_candid() -> String {
    export_service!();
    __export_service()
}

#[cfg(test)]
mod tests {
    use super::export_candid;
    #[test]
    fn _write_candid_to_disk() {
        std::fs::write("encrypted_notes_backend.did", export_candid()).expect("Write failed.");
    }
}
