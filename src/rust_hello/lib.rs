use ic_cdk::api::{trap};
use ic_cdk_macros::{post_upgrade, pre_upgrade, update};
use std::cell::RefCell;
use ic_cdk::export::candid::{candid_method, CandidType, Deserialize, Nat};
use num_traits::ToPrimitive;

#[derive(CandidType, Deserialize, Clone)]
struct NftInfo {
    nft_data: Vec<u8>,
}

impl Default for NftInfo {
    fn default() -> Self {
        NftInfo {
            nft_data: vec![]
        }
    }
}

thread_local!(
    /* stable */   static NFTINFO: RefCell<NftInfo> = RefCell::new(NftInfo::default());
);

#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}


#[update(name = "uploadData")]
#[candid_method(update, rename = "uploadData")]
fn upload_data(num : Nat) -> Nat {
    let num = num.0.to_u32().unwrap();
    let data:Vec::<u8> = vec!(3; (num * 1024 * 1024) as usize);
    NFTINFO.with(|nft_info| {
        // nft_info.borrow_mut().nft_data.append(data);
        nft_info.borrow_mut().nft_data.extend(data);
        Nat::from(nft_info.borrow().nft_data.len())
    })
}


// NOTE:
// If you plan to store gigabytes of state and upgrade the code,
// Using stable memory as the main storage is a good option to consider
#[pre_upgrade]
fn pre_upgrade() {
    if let Err(err) = ic_cdk::storage::stable_save::<(NftInfo, )>((
        NFTINFO.with(|nft_info| nft_info.borrow().clone()),
    )) {
        // NOTE: be careful and make sure it will never trap
        trap(&format!(
            "An error occurred when saving to stable memory (pre_upgrade): {:?}",
            err
        ));
    };
}

#[post_upgrade]
fn post_upgrade() {
    match ic_cdk::storage::stable_restore::<(NftInfo, )>() {
        Ok((nft_store, )) => {
            NFTINFO.with(|ledger| {
                *ledger.borrow_mut() = nft_store;
            });
        }
        Err(err) => {
            trap(&format!(
                "An error occurred when loading from stable memory (post_upgrade): {:?}",
                err
            ));
        }
    }
}