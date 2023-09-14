use crate::devices::*;
use crate::notes::*;
use ic_cdk::api::caller as caller_api;
use ic_cdk::export::candid::{candid_method, export_service};
use ic_cdk::export::Principal;
use ic_cdk_macros::*;
use std::cell::RefCell;

mod devices;
mod notes;

thread_local! {
    static DEVICES: RefCell<Devices> = RefCell::default();
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

fn is_caller_registered(caller: Principal) -> bool {
    DEVICES.with(|devices| devices.borrow().devices.contains_key(&caller))
}

#[candid_method(update)]
#[update(name = "registerDevice")]
fn register_device(alias: DeviceAlias, public_key: PublicKey) {
    let caller = caller();

    DEVICES.with(|devices| {
        devices
            .borrow_mut()
            .register_device(caller, alias, public_key)
    })
}

#[candid_method(query)]
#[query(name = "getDeviceAliases")]
fn get_device_aliases() -> Vec<DeviceAlias> {
    let caller = caller();
    assert!(is_caller_registered(caller));

    DEVICES.with(|devices| devices.borrow().get_device_aliases(caller))
}

#[candid_method(update)]
#[update(name = "deleteDevice")]
fn delete_device(alias: DeviceAlias) {
    let caller = caller();
    assert!(is_caller_registered(caller));

    DEVICES.with(|devices| {
        devices.borrow_mut().delete_device(caller, alias);
    })
}

#[candid_method(query)]
#[query(name = "getNotes")]
fn get_notes() -> Vec<EncryptedNote> {
    let caller = caller();
    assert!(is_caller_registered(caller));
    NOTES.with(|notes| notes.borrow().get_notes(caller))
}

#[candid_method(update)]
#[update(name = "addNote")]
fn add_note(encrypted_text: String) {
    let caller = caller();
    assert!(is_caller_registered(caller));
    NOTES.with(|notes| {
        notes.borrow_mut().add_note(caller, encrypted_text);
    })
}

#[candid_method(update)]
#[update(name = "deleteNote")]
fn delete_note(id: u128) {
    let caller = caller();
    assert!(is_caller_registered(caller));
    NOTES.with(|notes| {
        notes.borrow_mut().delete_note(caller, id);
    })
}

#[candid_method(update)]
#[update(name = "updateNote")]
fn update_note(new_note: EncryptedNote) {
    let caller = caller();
    assert!(is_caller_registered(caller));
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
