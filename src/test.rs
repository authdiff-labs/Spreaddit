#![cfg(test)]

use super::*;
extern crate std;
use soroban_sdk::Bytes;


#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, SpredditContract);
    let client = SpredditContractClient::new(&env, &contract_id);

    let state = client.refr_state();
    assert_eq!(state.articles.len(), 0);

    let uri = Bytes::from_slice(&env, "https://synced.to".as_bytes());
    let description = Bytes::from_slice(&env, "Crowdsourced geospatial data".as_bytes());

    let new_state = client.vote(&uri, &100, &description);
    let article = new_state.articles.get_unchecked(uri.clone()).unwrap();
    assert_eq!(article.count, 100);
    assert_eq!(article.descr, description);
    assert_eq!(article.uri, uri);

    let after_downvote = client.vote(&uri, &-30, &Bytes::from_slice(&env, "".as_bytes()));
    let updated = after_downvote.articles.get_unchecked(uri.clone()).unwrap();
    assert_eq!(updated.count, 70);

    let removed = client.vote(&uri, &-1000, &Bytes::from_slice(&env, "".as_bytes()));
    assert!(!removed.articles.contains_key(uri.clone()));
    assert_eq!(client.article_count(), 0);
}

#[test]
fn ignores_zero_and_empty_uri() {
    let env = Env::default();
    let contract_id = env.register_contract(None, SpredditContract);
    let client = SpredditContractClient::new(&env, &contract_id);

    let uri = Bytes::from_slice(&env, "https://example.com".as_bytes());
    let descr = Bytes::from_slice(&env, "test".as_bytes());
    let unchanged = client.vote(&uri, &0, &descr);
    assert_eq!(unchanged.articles.len(), 0);

    let empty = Bytes::from_slice(&env, "".as_bytes());
    let unchanged_empty = client.vote(&empty, &10, &descr);
    assert_eq!(unchanged_empty.articles.len(), 0);
}
